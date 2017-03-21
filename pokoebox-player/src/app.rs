use action::action_manager::ActionManager;
use error::Error;
use gui::gui::Gui;
use perif::perif_manager::PerifManager;

/// Base application struct.
pub struct App {
    /// Gui.
    gui: Gui,

    /// Action manager
    action_manager: ActionManager,

    /// Peripherals manager.
    perif_manager: PerifManager,
}

impl App {
    /// Create a new app instance.
    pub fn new() -> Result<Self, Error> {
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
    /// This will create things like the GUI, and starts initialization of all peripherals.
    pub fn start(&mut self) -> Result<(), Error> {
        // Start the GUI
        self.gui.start();

        // Load the normal actions
        self.action_manager.load_normal_actions();

        // TODO: Start the peripheral manager here!

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

    /// Run the main loop of the application.
    pub fn main_loop(&self) {
        self.gui.main_loop();
    }
}