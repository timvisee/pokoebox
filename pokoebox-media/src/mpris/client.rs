use std::collections::HashMap;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::{Duration, Instant};

use mpris::PlayerFinder;
use pokoebox_common::pipe::Pipe;

use super::{
    player::{Player, PlayerHandle},
    util, Cmd, Event,
};

/// Automatically refresh MPRIS players at this interval.
const MPRIS_PLAYER_REFRESH_INTERVAL: Duration = Duration::from_secs(30);

pub(crate) struct Client
where
    Self: Send + Sync,
{
    pub events: Pipe<Event>,
    pub(crate) cmds: Pipe<Cmd>,
}

impl Client {
    /// Construct new MPRIS client.
    pub fn new() -> Self {
        let (pipe_event, pipe_cmd) = InnerClient::spawn_thread();

        Self {
            events: pipe_event,
            cmds: pipe_cmd,
        }
    }
}

struct InnerClient {
    /// Events pipe, from inner client.
    events: Pipe<Event>,

    /// Commands pipe, to inner client.
    cmds: Pipe<Cmd>,

    /// Finder, to find new players.
    finder: PlayerFinder,

    /// List of players, with internal MPRIS players.
    mpris_players: HashMap<PlayerHandle, mpris::Player<'static>>,

    /// List of players, with external player state.
    players: HashMap<PlayerHandle, Player>,

    /// Last time the MPRIS player list was refreshed.
    last_player_refresh: Instant,
}

impl InnerClient {
    fn new(events: Pipe<Event>, cmds: Pipe<Cmd>) -> Self {
        // TODO: propagate error
        Self {
            events,
            cmds,
            finder: PlayerFinder::new().expect("failed to connect to DBus for MPRIS"),
            mpris_players: HashMap::new(),
            players: HashMap::new(),
            last_player_refresh: Instant::now(),
        }
    }

    fn spawn_thread() -> (Pipe<Event>, Pipe<Cmd>) {
        let events = Pipe::default();
        let cmds = Pipe::default();
        let out_events = events.clone();
        let out_cmds = cmds.clone();

        let (ready_tx, ready) = mpsc::channel();

        // Control mixer in thread
        thread::spawn(move || {
            // Construct inner device mixer
            let mut inner = Self::new(events, cmds);

            inner.run(ready_tx);
        });

        // Wait for readyness
        ready
            .recv()
            .expect("Failed to wait for MPRIS worker thread to become ready");

        (out_events, out_cmds)
    }

    fn run(&mut self, ready: Sender<()>) {
        let cmd_rx = self.cmds.listen();

        // Notify parent that we're ready
        ready
            .send(())
            .expect("Failed to signal readyness state of MPRIS worker thread");

        loop {
            // Get new command
            // TODO: add timeout, periodically poll for new MPRIS players
            let cmd = match cmd_rx.recv_timeout(MPRIS_PLAYER_REFRESH_INTERVAL) {
                Err(_) => {
                    // Queue command to find new players on interval
                    if self.last_player_refresh.elapsed() >= MPRIS_PLAYER_REFRESH_INTERVAL {
                        if let Err(err) = self.cmds.send(Cmd::FindPlayers) {
                            error!("Failed to queue command to find new MPRIS players at interval: {:?}", err);
                        }
                    }

                    // TODO: break on disconnect, continue on timeout
                    continue;
                }
                Ok(cmd) => cmd,
            };

            // Handle command
            match cmd {
                Cmd::FindPlayers => {
                    debug!("Refreshing list of available MPRIS players...");

                    // Find players, put in hashmap
                    let mut players: HashMap<PlayerHandle, _> = match self.finder.find_all() {
                        Ok(players) => players
                            .into_iter()
                            .map(|p| (PlayerHandle::from(&p), p))
                            .collect(),
                        Err(err) => {
                            error!("Failed to find MPRIS players: {:?}", err);
                            continue;
                        }
                    };

                    // Find diff with current list
                    let (add, remove) = util::iter_diff(
                        self.mpris_players.keys().cloned().collect(),
                        &players.keys().cloned().collect::<Vec<_>>(),
                    );

                    // Update list, emit change events
                    for handle in add {
                        let mpris_player = players.remove(&handle).unwrap();
                        let player = Player::from(&mpris_player)
                            .expect("Failed to abstract player from MPRIS player");
                        self.mpris_players.insert(handle.clone(), mpris_player);
                        self.players.insert(handle.clone(), player.clone());

                        if let Err(err) = self.events.send(Event::AddPlayer(handle.clone(), player))
                        {
                            error!("Failed to send AddPlayer event: {:?}", err);
                        }
                    }
                    for handle in remove {
                        if let Err(err) = self.events.send(Event::RemovePlayer(handle.clone())) {
                            error!("Failed to send RemovePlayer event: {:?}", err);
                        }

                        self.mpris_players.remove(&handle);
                        self.players.remove(&handle);
                    }

                    // Emit last list of players
                    if let Err(err) = self
                        .events
                        .send(Event::Players(self.players.values().cloned().collect()))
                    {
                        error!("Failed to send Players event: {:?}", err);
                    }

                    // Update refresh time
                    self.last_player_refresh = Instant::now();
                }
                Cmd::Play => {
                    if let Some((_handle, player)) = self.mpris_players.iter().next() {
                        if let Err(err) = player.play() {
                            error!("Failed send play signal to MPRIS player: {:?}", err);
                        }
                    }
                }
                Cmd::Pause => {
                    if let Some((_handle, player)) = self.mpris_players.iter().next() {
                        if let Err(err) = player.pause() {
                            error!("Failed send pause signal to MPRIS player: {:?}", err);
                        }
                    }
                }
                Cmd::PlayPause => {
                    if let Some((_handle, player)) = self.mpris_players.iter().next() {
                        if let Err(err) = player.play_pause() {
                            error!("Failed send play/pause signal to MPRIS player: {:?}", err);
                        }
                    }
                }
                Cmd::Next => {
                    if let Some((_handle, player)) = self.mpris_players.iter().next() {
                        if let Err(err) = player.next() {
                            error!("Failed send next signal to MPRIS player: {:?}", err);
                        }
                    }
                }
                Cmd::Previous => {
                    if let Some((_handle, player)) = self.mpris_players.iter().next() {
                        if let Err(err) = player.previous() {
                            error!("Failed send previous signal to MPRIS player: {:?}", err);
                        }
                    }
                }
            }
        }
    }
}