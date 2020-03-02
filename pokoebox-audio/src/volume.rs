use std::thread;

use alsa::mixer::{Mixer, Selem, SelemChannelId, SelemId};
use alsa::Result as AlsaResult;
use pokoebox_common::pipe::{Error as PipeError, Pipe};

// TODO: do not hardcode this here
const ALSA_CARD_PREFER: &str = "Intel";
const DEFAULT_CHANNEL: SelemChannelId = SelemChannelId::FrontLeft;

/// Volume manager.
pub struct VolumeManager {
    // Device mixer.
    pub mixer: DeviceMixer,
}

impl VolumeManager {
    pub fn new() -> Self {
        Self {
            mixer: DeviceMixer::new(),
        }
    }

    /// Send command to the mixer.
    pub fn send_cmd(&self, cmd: Cmd) -> Result<(), PipeError> {
        self.mixer.cmds.send(cmd).map(|_| ())
    }

    /// Query list of controls, this is blocking.
    pub fn query_controls(&self) -> Result<Vec<ControlHandle>, PipeError> {
        let event_rx = self.mixer.events.listen();
        self.send_cmd(Cmd::GetControls)?;
        loop {
            match event_rx
                .recv()
                .expect("couldn't receive device mixer event")
            {
                Event::Controls(controls) => return Ok(controls),
                _ => {}
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Cmd {
    /// Reset volume to defaults.
    ResetVolume,

    /// Request list of controls.
    GetControls,

    /// Get volume of given control.
    GetVolume(ControlHandle),

    /// Set volume of given control.
    SetVolume(ControlHandle, i64),
}

#[derive(Clone, Debug)]
pub enum Event {
    /// List of all control handles.
    Controls(Vec<ControlHandle>),

    /// Current volume for control.
    Volume(ControlHandle, i64),
}

/// Device mixer.
///
/// Provides command/event interface, internally spawns background thread to manage device mixer.
pub struct DeviceMixer
where
    Self: Send + Sync,
{
    pub events: Pipe<Event>,
    cmds: Pipe<Cmd>,
}

impl DeviceMixer {
    /// Construct new device mixer.
    fn new() -> Self {
        let (pipe_event, pipe_cmd) = InnerDeviceMixer::spawn_thread();

        Self {
            events: pipe_event,
            cmds: pipe_cmd,
        }
    }

    // TODO: fn set_master_volume(&self, volume: i64) {}
}

struct InnerDeviceMixer {
    /// Events pipe, from device mixer.
    events: Pipe<Event>,

    /// Commands pipe, to device mixer.
    cmds: Pipe<Cmd>,

    /// Alsa mixer.
    mixer: Mixer,

    /// List of Alsa controls.
    controls: Vec<Control>,
}

impl InnerDeviceMixer {
    fn new() -> Self {
        // TODO: propagate error
        let mixer = Mixer::new(&select_card(), true).expect("failed to open mixer");

        // List available controls
        let controls = mixer
            .iter()
            .filter_map(|e| Selem::new(e))
            .filter(|e| e.has_playback_volume())
            .map(Control::new)
            .collect();

        Self {
            events: Pipe::default(),
            cmds: Pipe::default(),
            mixer,
            controls,
        }
    }

    fn spawn_thread() -> (Pipe<Event>, Pipe<Cmd>) {
        // Construct inner device mixer
        let mut inner = Self::new();
        let out_events = inner.events.clone();
        let out_cmds = inner.cmds.clone();

        // Control mixer in thread
        thread::spawn(move || {
            inner.run();
        });

        (out_events, out_cmds)
    }

    fn run(&mut self) {
        let cmd_rx = self.cmds.listen();

        loop {
            // Get new command
            let cmd = match cmd_rx.recv() {
                Err(_) => break,
                Ok(cmd) => cmd,
            };

            // Handle command
            match cmd {
                Cmd::GetControls => {
                    if let Err(err) = self.events.send(Event::Controls(
                        self.controls.iter().map(Control::handle).collect(),
                    )) {
                        error!("Failed to send event for control list: {:?}", err);
                    }
                }
                Cmd::ResetVolume => {
                    todo!("Reset volume");
                }
                Cmd::GetVolume(control) => {
                    let volume = self
                        .control(&control)
                        .get_volume(&self.mixer)
                        .expect("failed to get playback volume");
                    if let Err(err) = self.events.send(Event::Volume(control, volume)) {
                        error!("Failed to send event for volume change: {:?}", err);
                    }
                }
                Cmd::SetVolume(control, volume) => {
                    info!("Setting Alsa volume to: {}", volume);
                    if let Err(err) = self.control(&control).set_volume(&self.mixer, volume) {
                        error!("Failed to set playback volume: {:?}", err);
                    } else if let Err(err) = self.events.send(Event::Volume(control, volume)) {
                        error!("Failed to send event for volume change: {:?}", err);
                    }
                }
            }
        }
    }

    /// Find a control for the given handle.
    fn control(&self, handle: &ControlHandle) -> &Control {
        self.controls
            .iter()
            .find(|c| c.index == handle.index)
            .expect("invalid control handle, doesn't correspond to real control")
    }
}

#[derive(Clone, Debug)]
pub struct ControlHandle {
    /// Control index.
    index: u32,

    /// Control name.
    name: Option<String>,
}

impl ControlHandle {
    /// Get control name.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

struct Control {
    /// Control index.
    index: u32,

    /// Control name.
    name: Option<String>,

    /// Alsa element ID.
    selem: SelemId,
}

impl Control {
    /// Create control from Alsa Selem.
    pub fn new<'a>(selem: Selem<'a>) -> Self {
        let selem = selem.get_id();

        Self {
            index: selem.get_index(),
            name: selem.get_name().ok().map(|n| n.into()),
            selem,
        }
    }

    /// Get handle to this control.
    pub fn handle(&self) -> ControlHandle {
        ControlHandle {
            index: self.index,
            name: self.name.clone(),
        }
    }

    /// Get reference to Alsa Selem.
    pub fn selem<'a>(&self, mixer: &'a Mixer) -> Selem<'a> {
        // TODO: do not unwrap here
        mixer
            .find_selem(&self.selem)
            .expect("failed to get selem from Alsa mixer")
    }

    /// Get current volume.
    pub fn get_volume(&self, mixer: &Mixer) -> AlsaResult<i64> {
        self.selem(mixer).get_playback_volume(DEFAULT_CHANNEL)
    }

    /// Set current volume.
    pub fn set_volume(&self, mixer: &Mixer, volume: i64) -> AlsaResult<()> {
        self.selem(mixer).set_playback_volume_all(volume)
    }
}

/// Select sound card to use.
fn select_card() -> String {
    format!(
        "hw:{}",
        alsa::card::Iter::new()
            .filter_map(|c| c.ok())
            .filter_map(|c| {
                c.get_name()
                    .ok()
                    .map(|n| Some((c.get_index(), n)))
                    .unwrap_or(None)
            })
            .filter(|(_, n)| n.contains(ALSA_CARD_PREFER))
            .inspect(|(i, n)| info!("Selected sound card: {} (hw:{})", n, i))
            .map(|(i, _)| i)
            .next()
            .unwrap_or(0)
    )
}
