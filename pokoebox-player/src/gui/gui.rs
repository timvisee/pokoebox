use super::gtk;

use error::Error;

use super::master_container::MasterContainer;
use super::master_frame::MasterFrame;
use super::master_ui::MasterUi;
use super::page::Page;

/// Main gui object, which manages the graphical interface side of the application.
///
/// Creating a new instance will automatically initialize GTK.
pub struct Gui {
    /// Master frame holding the gui.
    master_frame: Option<MasterFrame>,

    /// Master container
    master_container: Option<MasterContainer>,

    /// Master UI
    master_ui: Option<MasterUi>
}

impl Gui {

    /// Constructor.
    ///
    /// Constructing the object will initialize the GTK toolkit.
    ///
    /// # Errors
    ///
    /// Returns an error if GTK failed to initialize, blocking further GTK usage.
    pub fn new() -> Result<Self, Error> {
        // Initialize GTK, handle errors
        if gtk::init().is_err() {
            return Err(Error::new("Failed to initialize GTK"));
        }

        // Build the object and return it
        Ok(Gui {
            master_frame: None,
            master_container: None,
            master_ui: None
        })
    }

    /// Start the gui.
    /// This sets up the master frame, and shows it.
    pub fn start(&mut self) {
        self.setup();
        self.show_master_frame();
    }

    /// Set up the main gui interface.
    /// This creates a window or frame, and builds the interface in it.
    /// Nothing happens if a master frame is already available.
    pub fn setup(&mut self) {
        // Do not set up if we already have a master frame
        if self.master_frame.is_some() {
            return;
        }

        // Create a master frame, container and ui
        let master_frame = MasterFrame::new();
        let master_container = MasterContainer::new();
        let mut master_ui = MasterUi::new();

        // Put the master container in the frame, and the master UI in the container
        master_frame.set_container(&master_container);
        master_container.set_ui(&master_ui);

        // Create and add a home page (test)
        let page = Page::new("Home");
        master_ui.mut_page_container().add_page(page);

        // Store the master frame, container and ui
        self.master_frame = Some(master_frame);
        self.master_container = Some(master_container);
        self.master_ui = Some(master_ui);
    }

    /// Get the master frame instance.
    /// `None` is returned if no master frame was created yet.
    pub fn master_frame(&self) -> Option<&MasterFrame> {
        match self.master_frame {
            Some(ref master_frame) => Some(master_frame),
            None => None
        }
    }

    /// Show the master frame and all it's inner widgets, if it's not already visible.
    /// If there's no master frame, nothing happens.
    pub fn show_master_frame(&self) {
        // Get the master frame
        let master_frame = self.master_frame();

        // Show it
        if master_frame.is_some() {
            master_frame.unwrap().show();
        }
    }
}