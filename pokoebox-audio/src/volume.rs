use std::collections::HashMap;
use std::thread;

use alsa::mixer::{Mixer, Selem, SelemChannelId, SelemId};
use alsa::Result as AlsaResult;
use pokoebox_common::pipe::{Error as PipeError, Pipe};

// TODO: do not hardcode this here
const ALSA_CARD_PREFER: &str = "Intel";
const DEFAULT_CHANNEL: SelemChannelId = SelemChannelId::FrontLeft;

/// Volume manager.
pub struct VolumeManager {
    /// Device mixer.
    pub mixer: DeviceMixer,

    /// Control properties.
    pub control_props: HashMap<ControlHandle, ControlProps>,
}

impl VolumeManager {
    pub fn new() -> Self {
        let mut mixer = Self {
            mixer: DeviceMixer::new(),
            control_props: HashMap::new(),
        };

        // List Alsa mixer controls
        mixer.control_props = mixer
            .query_controls()
            .expect("Failed to query Alsa mixer controls");

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

    /// Adjust volume of given control.
    AdjustVolume(ControlHandle, i64),
}

#[derive(Clone, Debug)]
pub enum Event {
    /// List of all control handles and properties.
    Controls(HashMap<ControlHandle, ControlProps>),

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
                    info!("Setting Alsa volume to: {}", volume);
                    if let Err(err) = self.control(&control).set_volume(&self.mixer, volume) {
                        error!("Failed to set playback volume: {:?}", err);
                    } else if let Err(err) = self.events.send(Event::Volume(control, volume)) {
                        error!("Failed to send event for volume change: {:?}", err);
                    }
                }
                Cmd::AdjustVolume(control, amount) => {
                    let volume = self.control(&control).get_volume(&self.mixer) + amount;
                    info!("Adjusting Alsa volume by {} to: {}", amount, volume);
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ControlHandle(u32, Option<String>);

#[derive(Clone, Debug)]
pub struct ControlProps {
    /// Control name.
    pub name: Option<String>,

    /// The value at initialization.
    pub init_value: i64,

    /// Control range.
    pub range: (i64, i64),
}

struct Control {
    /// Handle.
    handle: ControlHandle,

    /// Alsa element ID.
    selem: SelemId,

    /// Control properties.
    props: ControlProps,
}

impl Control {
    /// Create control from Alsa Selem.
    pub fn from_selem<'a>(selem: Selem<'a>) -> Self {
        let id = selem.get_id();
        let props = ControlProps {
            name: id.get_name().ok().map(|n| n.into()),
            init_value: selem
                .get_playback_volume(DEFAULT_CHANNEL)
                .expect("failed to query Alsa control volume"),
            range: selem.get_playback_volume_range(),
        };

        let handle = ControlHandle(id.get_index(), id.get_name().map(|n| n.into()).ok());

        Self {
            handle,
            selem: id,
            props,
        }
    }

    /// Get handle to this control.
    pub fn handle(&self) -> &ControlHandle {
        &self.handle
    }

    /// Get control properties.
    pub fn props(&self) -> &ControlProps {
        &self.props
    }

    /// Get reference to Alsa Selem.
    pub fn selem<'a>(&self, mixer: &'a Mixer) -> Selem<'a> {
        // TODO: do not unwrap here
        mixer
            .find_selem(&self.selem)
            .expect("failed to get selem from Alsa mixer")
    }

    /// Get current volume.
    pub fn get_volume(&self, mixer: &Mixer) -> i64 {
        self.selem(mixer)
            .get_playback_volume(DEFAULT_CHANNEL)
            .expect("failed to set Alsa control volume")
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
