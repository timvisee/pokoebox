use std::sync::Arc;

#[cfg(feature = "bluetooth")]
use pokoebox_bluetooth::manager::Manager as BluetoothManager;
#[cfg(feature = "rpi")]
use pokoebox_rpi::{
    button::{ButtonConfig, Interface as ButtonInterface},
    led::Interface as LedInterface,
};

use crate::action::{actions::GotoPageAction, ActionRuntime};
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

    fn setup_buttons(core: Arc<Core>) -> std::result::Result<(), pokoebox_rpi::button::Error> {
        // Set up buttons
        let closure_core = core.clone();
        core.buttons
            .setup_button(ButtonConfig::Push(27), move |_| {
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

        Ok(())
    }
}
