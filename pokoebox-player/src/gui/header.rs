use super::gtk;
use super::gtk::*;

/// Main UI header in the application.
pub struct Header {
    container: gtk::Box
}

impl Header {

    /// Construct a new header.
    pub fn new() -> Self {
        Header {
            container: Self::build_container()
        }
    }

    /// Build the header container.
    fn build_container() -> gtk::Box {
        // Create a container instance
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        // Configure the header
        container.set_hexpand(true);
        container.set_vexpand(false);
        container.set_halign(gtk::Align::Center);

        container
    }

    /// Get the GTK widget for this header.
    pub fn gtk_widget(&self) -> &gtk::Box {
        &self.container
    }
}