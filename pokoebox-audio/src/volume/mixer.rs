use std::thread;

use alsa::mixer::{Mixer, Selem};
use pokoebox_common::pipe::Pipe;

use super::control::{Control, ControlHandle};
use super::util;
use super::{Cmd, Event};

/// Device mixer.
///
/// Provides command/event interface, internally spawns background thread to manage device mixer.
pub struct DeviceMixer
where
    Self: Send + Sync,
{
    pub events: Pipe<Event>,
    pub(crate) cmds: Pipe<Cmd>,
}

impl DeviceMixer {
    /// Construct new device mixer.
    pub fn new() -> Self {
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
        let mixer = Mixer::new(&util::select_card(), true).expect("failed to open mixer");

        // List available controls
        let controls = mixer
            .iter()
            .filter_map(|e| Selem::new(e))
            .filter(|e| e.has_playback_volume())
            .map(Control::from_selem)
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
                        self.controls
                            .iter()
                            .map(|c| (c.handle().clone(), c.props().clone()))
                            .collect(),
                    )) {
                        error!("Failed to send event for control list: {:?}", err);
                    }
                }
                Cmd::ResetVolume => {
                    todo!("Reset volume");
                }
                Cmd::GetVolume(control) => {
                    let volume = self.control(&control).get_volume(&self.mixer);
                    if let Err(err) = self.events.send(Event::Volume(control, volume)) {
                        error!("Failed to send event for volume change: {:?}", err);
                    }
                }
                Cmd::SetVolume(control, volume) => {
                    // TODO: use return value on set?
                    if let Err(err) = self.control(&control).set_volume(&self.mixer, volume) {
                        error!("Failed to set playback volume: {:?}", err);
                    } else if let Err(err) = self.events.send(Event::Volume(control, volume)) {
                        error!("Failed to send event for volume change: {:?}", err);
                    }
                }
                Cmd::AdjustVolume(control, amount) => {
                    // TODO: use return value on set?
                    let volume = self.control(&control).get_volume(&self.mixer) + amount;
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
            .find(|c| c.handle() == handle)
            .expect("invalid control handle, doesn't correspond to real control")
    }
}
