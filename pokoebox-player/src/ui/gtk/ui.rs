use std::sync::Arc;

use crate::app::Core;
use crate::error::Error;
use crate::pages::PageType;
use crate::result::Result;

use super::main::App;
use super::Window;

/// Main gui object, which manages the graphical interface side of the
/// application.
///
/// Creating a new instance will automatically initialize GTK.
pub struct Ui {
    /// Main window.
    window: Window,

    /// Main app UI.
    _app_ui: App,
}

impl Ui {
    /// Constructor.
    ///
    /// Constructing the object will initialize the GTK toolkit.
    ///
    /// # Errors
    ///
    /// Returns an error if GTK failed to initialize,
    /// blocking further GTK usage.
    pub fn new(core: Arc<Core>) -> Result<Self> {
        // Initialize GTK
        debug!("Initializing GTK...");
        if gtk::init().is_err() {
            return Err(Error::new("Failed to initialize GTK"));
        }
        debug!("Successfully initialized GTK.");

        // Build UI
        let (window, _app_ui) = Self::build_ui(core);
        let gui = Self { window, _app_ui };

        // Show window
        info!("Showing master GUI frame...");
        gui.window.show();

        Ok(gui)
    }

    /// Set up the main gui interface.
    /// This creates a window or frame, and builds the interface in it.
    /// Nothing happens if a master frame is already available.
    fn build_ui(core: Arc<Core>) -> (Window, App) {
        // Create window and app UI
        let window = Window::new(core.clone());
        let app = App::new(core.clone());

        // Put app UI in window
        window.set_app(&app);

        // Show launchpad
        app.pages.manager.goto_page(core, PageType::Launchpad);

        (window, app)
    }

    /// Run the main loop of the GUI.
    pub fn main(&self) {
        gtk::main();
    }
}
