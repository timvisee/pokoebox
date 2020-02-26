use std::sync::Arc;

#[cfg(feature = "bluetooth")]
use pokoebox_bluetooth::manager::Manager as BluetoothManager;
#[cfg(feature = "rpi")]
use pokoebox_rpi::led::Controller as LedController;

use crate::action::ActionRuntime;
use crate::result::Result;
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

    /// LED manager.
    #[cfg(feature = "rpi")]
    pub led: LedController,

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
            led: LedController::new().expect("failed to initialize LED controller"),
            pages: PageController::new(),
        })
    }
}
