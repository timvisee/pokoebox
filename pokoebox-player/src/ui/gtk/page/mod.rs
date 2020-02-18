mod container;

use gtk::{self, prelude::*};

pub use container::Container;

/// Page trait.
/// This trait is used for a page implementation.
/// It builds and manages the page.
pub trait Page {
    /// Get the name of the page.
    fn page_name(&self) -> &'static str;

    /// Build the actual page gui on the GTK widget of the given page.
    fn build_page(&self);

    /// Get the GTK widget that represents the page.
    fn gtk_widget(&self) -> &gtk::Grid;
}

/// Page helper struct.
pub struct Helper {}

impl Helper {
    /// Create a new GTK page container,
    /// that may be used to build a new page upon.
    /// The container is partially configured to show the page on.
    pub fn create_page_container() -> gtk::Grid {
        // Create the page container
        let container = gtk::Grid::new();

        // Configure the container
        container.set_hexpand(true);
        container.set_vexpand(true);
        container.set_border_width(8);

        container
    }
}
