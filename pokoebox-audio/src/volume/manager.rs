use std::collections::HashMap;

use pokoebox_common::pipe::Error as PipeError;

use super::control::{ControlHandle, ControlProps};
use super::mixer::DeviceMixer;
use super::{Cmd, Event};

/// Volume manager.
pub struct Manager {
    /// Device mixer.
    pub mixer: DeviceMixer,

    /// Control properties.
    pub control_props: HashMap<ControlHandle, ControlProps>,
}

impl Manager {
    pub fn new() -> Self {
        let mut mixer = Self {
            mixer: DeviceMixer::new(),
            control_props: HashMap::new(),
        };

        // List Alsa mixer controls
        match mixer.query_controls() {
            Ok(controls) => {
                mixer.control_props = controls;
            }
            Err(err) => {
                error!("Failed to query volume mixer controls: {:?}", err);
            }
        }

        mixer
    }

    /// Send command to the mixer.
    pub fn send_cmd(&self, cmd: Cmd) -> Result<(), PipeError> {
        self.mixer.cmds.send(cmd).map(|_| ())
    }

    /// Query list of controls, this is blocking.
    pub fn query_controls(&self) -> Result<HashMap<ControlHandle, ControlProps>, PipeError> {
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

    /// Find the master control.
    // TODO: propagate errors here
    pub fn get_master_control(&self) -> (&ControlHandle, &ControlProps) {
        self.control_props
            .iter()
            .find(|(_, p)| {
                p.name
                    .as_ref()
                    .map(|n| n.contains("Digital"))
                    .unwrap_or(false)
            })
            .unwrap_or_else(|| {
                self.control_props
                    .iter()
                    .next()
                    .expect("Could not find master volume control")
            })
    }
}
