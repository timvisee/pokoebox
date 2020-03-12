use std::sync::Arc;

use glib::clone;
use pokoebox_audio::volume::Manager as VolumeManager;
#[cfg(feature = "bluetooth")]
use pokoebox_bluetooth::manager::Manager as BluetoothManager;
use pokoebox_common::pipe::Pipe;
use pokoebox_media::{mpris::Manager as MprisManager, player::Player};
#[cfg(feature = "rpi")]
use pokoebox_rpi::{
    button::{ButtonConfig, Event as ButtonEvent, Interface as ButtonInterface},
    led::{Interface as LedInterface, Led},
    power::Interface as PowerInterface,
    rpi::Rpi,
};

use crate::action::{
    actions::{AdjustVolume, GotoPageAction},
    ActionRuntime,
};
use crate::message::Message;
use crate::pages::PageType;
use crate::result::Result;
use crate::soundeffecter::SoundEffecter;
use crate::ui::gtk::Ui;

use super::pages::PageController;

pub struct App {
    ui: Ui,
    pub core: Arc<Core>,
}

impl App {
    pub fn new() -> Result<Self> {
        // Init app core
        let core = Arc::new(Core::new()?);

        #[cfg(feature = "rpi")]
        Core::setup_buttons(core.clone()).expect("Failed to set-up app buttons");

        let app = Self {
            ui: Ui::new(core.clone())?,
            core,
        };

        Ok(app)
    }

    pub fn run(self) -> Self {
        // Run the GUIs main loop
        loop {
            self.ui.main();
        }
    }
}

pub struct Core {
    /// Messages pipe.
    pub messages: Pipe<Message>,

    /// Action manager
    pub actions: ActionRuntime,

    /// Player controller.
    pub player: Player,

    /// Volume manager.
    pub volume: VolumeManager,

    /// MPRIS manager.
    pub mpris: MprisManager,

    /// Bluetooth manager.
    #[cfg(feature = "bluetooth")]
    pub bluetooth: BluetoothManager,

    /// LED interface.
    #[cfg(feature = "rpi")]
    pub leds: LedInterface,

    /// Button interface.
    #[cfg(feature = "rpi")]
    pub buttons: ButtonInterface,

    /// Power interface.
    #[cfg(feature = "rpi")]
    pub power: PowerInterface,

    /// Sound effecter.
    pub effecter: SoundEffecter,

    pub pages: PageController,
}

impl Core {
    /// Construct a new core.
    pub fn new() -> Result<Self> {
        // Construct RPi base to share resources
        #[cfg(feature = "rpi")]
        let mut rpi = Rpi::default();

        Ok(Self {
            messages: Pipe::default(),
            actions: ActionRuntime::default(),
            player: Player::default(),
            volume: VolumeManager::new(),
            mpris: MprisManager::new(),
            // TODO: propagate error
            #[cfg(feature = "bluetooth")]
            bluetooth: BluetoothManager::new().expect("failed to initialize bluetooth manager"),
            // TODO: propagate error
            #[cfg(feature = "rpi")]
            leds: LedInterface::new(&mut rpi).expect("failed to initialize LED interface"),
            // TODO: propagate error
            #[cfg(feature = "rpi")]
            buttons: ButtonInterface::new().expect("failed to initialize button interface"),
            // TODO: propagate error
            #[cfg(feature = "rpi")]
            power: PowerInterface::new(&mut rpi).expect("failed to initialize power interface"),
            // TODO: propagate error
            effecter: SoundEffecter::new().expect("failed to initialize sound effecter"),
            pages: PageController::new(),
        })
    }

    #[cfg(feature = "rpi")]
    fn setup_buttons(core: Arc<Core>) -> std::result::Result<(), pokoebox_rpi::button::Error> {
        // Set up buttons
        core.buttons.setup_button(
            ButtonConfig::Push(17),
            clone!(@weak core => move |_| {
                if let Err(err) = core
                    .mpris
                    .send_cmd(pokoebox_media::mpris::Cmd::PlayPause)
                {
                    error!(
                        "Failed to send play/pause signal to MPRIS player: {:?}",
                        err
                    );
                }
            }),
        )?;

        core.buttons.setup_button(
            ButtonConfig::Push(27),
            clone!(@weak core => move |_| {
                if let Err(err) = core
                    .mpris
                    .send_cmd(pokoebox_media::mpris::Cmd::Next)
                {
                    error!("Failed to send next signal to MPRIS player: {:?}", err);
                }
            }),
        )?;

        core.buttons.setup_button(
            ButtonConfig::Push(5),
            clone!(@weak core => move |_| {
                core.actions.invoke(
                    GotoPageAction::new(PageType::Launchpad),
                    core.clone(),
                );
            }),
        )?;

        #[cfg(feature = "bluetooth")]
        {
            core.buttons.setup_button(
                ButtonConfig::Push(6),
                clone!(@weak core => move |_| {
                    core.actions.invoke(
                        GotoPageAction::new(PageType::Bluetooth),
                        core.clone(),
                    );
                }),
            )?;

            core.buttons.setup_button(
                ButtonConfig::Push(13),
                clone!(@weak core => move |_| {
                    if let Err(err) = core.bluetooth.set_discoverable(true) {
                        error!("Failed to send bluetooth discover command: {:?}", err);
                    }
                }),
            )?;
        }

        core.buttons.setup_button(
            ButtonConfig::Rotary(23, 24),
            clone!(@weak core => move |event| {
                let action = match event {
                    ButtonEvent::Up => AdjustVolume::up(),
                    ButtonEvent::Down => AdjustVolume::down(),
                    _ => return,
                };
                core.actions.invoke(action, core.clone());
            }),
        )?;

        // TODO: move somewhere else
        #[cfg(feature = "bluetooth")]
        {
            core.bluetooth
                .events
                .register_callback(clone!(@weak core => move |event| {
                    if let pokoebox_bluetooth::manager::Event::Discoverable(status) = event {
                        if let Err(err) = core.leds.led_set(Led::Action4, status) {
                            error!("Failed to set bluetooth status LED: {:?}", err);
                        }
                        if let Err(err) = core.leds.led_set(Led::PowerButton, status) {
                            error!("Failed to set bluetooth status LED: {:?}", err);
                        }
                    }
                }));
        }

        // TODO: move somewhere else
        #[cfg(feature = "bluetooth")]
        core.bluetooth.events.register_callback(clone!(@weak core => move |event| {
            use pokoebox_bluetooth::manager::Event;
            if let Event::DeviceConnected(_, _) | Event::DeviceDisconnected(_, _) = event {
                if let Err(err) = core.mpris.send_cmd(pokoebox_media::mpris::Cmd::FindPlayers) {
                    error!(
                        "Failed to send command to MPRIS manager to find available players: {:?}",
                        err
                    );
                }
            }
        }));

        // Add new source on new MPRIS player
        core.mpris
            .events()
            .register_callback(clone!(@weak core => move |event| {
                use pokoebox_media::mpris::Event;
                match event {
                    Event::AddPlayer(handle, player) => {
                        // TODO: create new source

                        let source = Box::new(pokoebox_media::player::sources::MprisSource::from(handle, player));

                        core.player.sources.lock().expect("failed to obtain lock on player sources").add(source);
                    }
                    Event::RemovePlayer(handle) => {
                        core.player.sources.lock().expect("failed to obtain lock on player sources").remove_remote_handle(&pokoebox_media::player::SourceRemoteHandle::Mpris(handle));
                    }
                    Event::Players(_) | Event::TrackInfo(_) => {}
               }
            }));

        Ok(())
    }

    /// Show a message to the user.
    pub fn show_message(&self, msg: Message) {
        self.messages.send(msg).expect("Failed to send message");
    }
}
