use super::gtk;
use super::gtk::*;

/// The actual UI inside the master container, with a header and page container to show the application's UI in.
pub struct MasterUi {
    header: gtk::Box,
    page_container: gtk::Notebook
}

impl MasterUi {

    /// Construct a new master UI.
    pub fn new() -> Self {
        MasterUi {
            header: Self::build_header(),
            page_container: Self::build_page_container()
        }
    }

    /// Build the header.
    fn build_header() -> gtk::Box {
        // Create a header instance
        let header = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        // Configure the header
        header.set_hexpand(true);
        header.set_vexpand(false);
        header.set_halign(gtk::Align::Center);

        header
    }

    /// Build the page container.
    fn build_page_container() -> gtk::Notebook {
        // Create the page container instance
        let page_container = gtk::Notebook::new();

        // Configure the page container
        page_container.set_hexpand(true);
        page_container.set_vexpand(true);
        page_container.set_halign(gtk::Align::Fill);
        page_container.set_valign(gtk::Align::Fill);

        page_container
    }

    /// Get the GTK header widget.
    pub fn header(&self) -> &gtk::Box {
        &self.header
    }

    /// Get the GTK page container widget.
    pub fn page_container(&self) -> &gtk::Notebook {
        &self.page_container
    }
}
