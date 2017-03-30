use action::action_id::ActionId;
use action::action_manager::ActionManager;
#[cfg(feature = "rpi")]
use gpio::gpio_manager::GpioManager;
use gui::gui::Gui;
use perif::perif_manager::PerifManager;
use result::Result;

/// Base application struct.
pub struct App {
    /// Gui.
    gui: Gui,

    /// Action manager
    action_manager: ActionManager,

    /// Peripherals manager.
    perif_manager: PerifManager,

    /// GPIO manager.
    #[cfg(feature = "rpi")]
    gpio_manager: GpioManager
}

impl App {
    /// Create a new app instance.
    #[cfg(feature = "rpi")]
    pub fn new() -> Result<Self> {
        debug!("Initializing application core...");

        // Create the application instance
        let app = App {
            gui: Gui::new()?,
            action_manager: ActionManager::new(),
            perif_manager: PerifManager::new(),
            gpio_manager: GpioManager::new()?,
        };

        debug!("Application core initialized.");
        Ok(app)
    }
    #[cfg(not(feature = "rpi"))]
    pub fn new() -> Result<Self> {
        debug!("Initializing application core...");

        // Create the application instance
        let app = App {
            gui: Gui::new()?,
            action_manager: ActionManager::new(),
            perif_manager: PerifManager::new(),
        };

        debug!("Application core initialized.");
        Ok(app)
    }

    /// Start the application.
    /// This will create things like the GUI,
    /// and starts initialization of all peripherals.
    pub fn start(&mut self) -> Result<()> {
        // Load the normal actions
        self.action_manager.load_normal_actions();

        // TODO: Remove this testing code
        // Run the test action
        self.action_manager.invoke_action(
            ActionId::new("test-action")
        ).unwrap();

        // TODO: Start the peripheral manager here!

        // Start the GUI
        self.gui.start();

        // Start the GPIO polling thread
        #[cfg(feature = "rpi")]
        self.gpio_manager.start_poll_thread();

        Ok(())
    }

    /// Get the GUI.
    pub fn gui(&self) -> &Gui {
        &self.gui
    }

    /// Get the peripherals manager.
    pub fn perif_manager(&self) -> &PerifManager {
        &self.perif_manager
    }

    /// Get the GPIO manager.
    #[cfg(feature = "rpi")]
    pub fn gpio_manager(&self) -> &GpioManager {
        &self.gpio_manager
    }

    /// Run the main loop of the application.
    pub fn main_loop(&self) {
        loop {
            // Run the GUIs main loop
            self.gui.main_loop();
        }
    }
}
