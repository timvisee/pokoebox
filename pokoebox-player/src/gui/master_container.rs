use super::gtk;

/// The master container, that holds all the applications GUI elements.
pub struct MasterContainer {
    /// Grid that is the container
    container: gtk::Grid
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
        gtk::Grid::new()
    }

    /// Get the GTK widget.
    pub fn gtk_widget(&self) -> &gtk::Grid {
        &self.container
    }
}