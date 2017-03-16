use super::gtk;
use super::gtk::*;

use super::master_container::MasterContainer;

/// The master frame, which creates a window or frame depending on the use case, to show the
/// master container in.
pub struct MasterFrame {
    /// Frame window.
    window: gtk::Window
}

impl MasterFrame {

    /// Create a new master frame.
    pub fn new() -> Self {
        MasterFrame {
            window: Self::build_window()
        }
    }

    /// Build the master frame window, and it's main container.
    /// Returns the created window.
    fn build_window() -> gtk::Window {
        // Create a window instance
        let window = gtk::Window::new(gtk::WindowType::Toplevel);

        // Configure the window
        window.set_title("PokoeBox Player");
        window.set_border_width(0);
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(640, 480);

        // Connect the window delete event
        window.connect_delete_event(|_, _| {
            // Close the application
            gtk::main_quit();

            // Leave window destruction to GTK
            Inhibit(false)
        });

        window
    }

    /// Set the container that is shown in this frame.
    pub fn set_container(&self, container: &MasterContainer) {
        // TODO: Should we remove the current child(ren) to replace them?

        self.window.add(container.gtk_widget());
    }

    /// Show the master frame and all it's widgets.
    pub fn show(&self) {
        self.window.show_all();
    }
}