use gui::gui::Gui;
use perif::perif_manager::PerifManager;

/// Base application struct.
pub struct App {
    /// Gui.
    gui: Gui,

    /// Peripherals manager.
    perif_manager: PerifManager,
}

impl App {
    /// Create a new app instance.
    pub fn new() -> Self {
        App {
            gui: Gui::new().unwrap(),
            perif_manager: PerifManager::new()
        }
    }

    /// Get the GUI.
    pub fn gui(&self) -> &Gui {
        &self.gui
    }

    /// Get the peripherals manager.
    pub fn perif_manager(&self) -> &PerifManager {
        &self.perif_manager
    }
}