use super::gtk;
use super::gtk::*;

use super::master_ui::MasterUi;

/// The master container, that holds all the applications GUI elements.
pub struct MasterContainer {
    /// Grid that is the container
    container: gtk::Grid,
}

impl MasterContainer {

    /// Create a new master container.
    pub fn new() -> Self {
        MasterContainer {
            container: Self::build_container()
        }
    }

    /// Build the main container.
    /// Returns the created container.
    fn build_container() -> gtk::Grid {
        // Build the grid
        let grid = gtk::Grid::new();

        // Configure the grid
        grid.set_hexpand(true);
        grid.set_vexpand(true);
        grid.set_halign(gtk::Align::Fill);
        grid.set_valign(gtk::Align::Fill);

        grid
    }

    /// Get the GTK grid.
    pub fn gtk_grid(&self) -> &gtk::Grid {
        &self.container
    }

    /// Set the master UI in this container.
    pub fn set_ui(&self, master_ui: &MasterUi) {
        // TODO: Should we remove all current children?

        // Add the master UI components
        self.container.attach(master_ui.header(), 0, 0, 1, 1);
        self.container.attach(master_ui.page_container(), 0, 1, 1, 1);
    }
}