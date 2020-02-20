use std::sync::Arc;

use gtk::prelude::*;

use crate::app::Core;
use crate::ui::gtk::page::Container;

use super::Header;

/// Main application UI and layout.
pub struct App {
    /// Page container.
    pub pages: Container,

    /// Layout grid.
    pub grid: gtk::Grid,
}

impl App {
    /// Create new app UI.
    pub fn new(core: Arc<Core>) -> Self {
        // Build container
        let grid = Self::build_container();

        // Build header and pages
        let header = Header::new(core.clone());
        let pages = Container::new(core);

        // Add the master UI components
        grid.attach(header.gtk_widget(), 0, 0, 1, 1);
        grid.attach(pages.gtk_widget(), 0, 1, 1, 1);

        Self { pages, grid }
    }

    /// Build main layout grid.
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
}
