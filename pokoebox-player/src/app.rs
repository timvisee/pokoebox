use std::sync::Arc;

use pokoebox_audio::volume::Manager as VolumeManager;
#[cfg(feature = "bluetooth")]
use pokoebox_bluetooth::manager::Manager as BluetoothManager;
use pokoebox_media::mpris::Manager as MprisManager;
#[cfg(feature = "rpi")]
use pokoebox_rpi::{
    button::{ButtonConfig, Event as ButtonEvent, Interface as ButtonInterface},
    led::{Interface as LedInterface, Led},
};

use crate::action::{
    actions::{AdjustVolume, GotoPageAction},
    ActionRuntime,
};
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

        Ok(Self {
            ui: Ui::new(core.clone())?,
            core,
        })
    }

    pub fn run(self) -> Self {
        // Run the GUIs main loop
        loop {
            self.ui.main();
        }
    }
}

pub struct Core {
    /// Action manager
    pub actions: ActionRuntime,

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

    /// Sound effecter.
    pub effecter: SoundEffecter,

    pub pages: PageController,
}

impl Core {
    pub fn new() -> Result<Self> {
        Ok(Self {
            actions: ActionRuntime::default(),
            volume: VolumeManager::new(),
            mpris: MprisManager::new(),
            // TODO: propagate error
            #[cfg(feature = "bluetooth")]
            bluetooth: BluetoothManager::new().expect("failed to initialize bluetooth manager"),
            // TODO: propagate error
            #[cfg(feature = "rpi")]
            leds: LedInterface::new().expect("failed to initialize LED interface"),
            // TODO: propagate error
            #[cfg(feature = "rpi")]
            buttons: ButtonInterface::new().expect("failed to initialize button interface"),
            // TODO: propagate error
            effecter: SoundEffecter::new().expect("failed to initialize sound effecter"),
            pages: PageController::new(),
        })
    }

    #[cfg(feature = "rpi")]
    fn setup_buttons(core: Arc<Core>) -> std::result::Result<(), pokoebox_rpi::button::Error> {
        // Set up buttons
        let closure_core = core.clone();
        core.buttons
            .setup_button(ButtonConfig::Push(17), move |_| {
                if let Err(err) = closure_core
                    .mpris
                    .send_cmd(pokoebox_media::mpris::Cmd::PlayPause)
                {
                    error!(
                        "Failed to send play/pause signal to MPRIS player: {:?}",
                        err
                    );
                }
            })?;

        let closure_core = core.clone();
        core.buttons
            .setup_button(ButtonConfig::Push(27), move |_| {
                if let Err(err) = closure_core
                    .mpris
                    .send_cmd(pokoebox_media::mpris::Cmd::Next)
                {
                    error!("Failed to send next signal to MPRIS player: {:?}", err);
                }
            })?;

        let closure_core = core.clone();
        core.buttons.setup_button(ButtonConfig::Push(5), move |_| {
            if let Err(err) = closure_core.actions.invoke(
                GotoPageAction::new(PageType::Launchpad),
                closure_core.clone(),
            ) {
                error!(
                    "Failed to goto launchpad page after button press: {:?}",
                    err
                );
            }
        })?;

        let closure_core = core.clone();
        core.buttons.setup_button(ButtonConfig::Push(5), move |_| {
            if let Err(err) = closure_core.actions.invoke(
                GotoPageAction::new(PageType::Bluetooth),
                closure_core.clone(),
            ) {
                error!(
                    "Failed to goto bluetooth page after button press: {:?}",
                    err
                );
            }
        })?;

        let closure_core = core.clone();
        core.buttons
            .setup_button(ButtonConfig::Push(13), move |_| {
                if let Err(err) = closure_core.bluetooth.set_discoverable(true) {
                    error!("Failed to send bluetooth discover command: {:?}", err);
                }
            })?;

        let closure_core = core.clone();
        core.buttons
            .setup_button(ButtonConfig::Rotary(23, 24), move |event| {
                let action = match event {
                    ButtonEvent::Up => AdjustVolume::up(),
                    ButtonEvent::Down => AdjustVolume::down(),
                    _ => return,
                };
                if let Err(err) = closure_core.actions.invoke(action, closure_core.clone()) {
                    error!("Failed to change volume: {:?}", err);
                }
            })?;

        // TODO: move somewhere else
        #[cfg(feature = "bluetooth")]
        core.clone()
            .bluetooth
            .events
            .register_callback(move |event| {
                if let pokoebox_bluetooth::manager::Event::Discoverable(status) = event {
                    if let Err(err) = core.leds.led_set(Led::Action4, status) {
                        error!("Failed to set bluetooth status LED: {:?}", err);
                    }
                    if let Err(err) = core.leds.led_set(Led::PowerButton, status) {
                        error!("Failed to set bluetooth status LED: {:?}", err);
                    }
                }
            });

        Ok(())
    }
}
