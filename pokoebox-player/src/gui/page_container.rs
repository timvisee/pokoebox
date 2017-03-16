use super::gtk;
use super::gtk::*;

/// Container holding and managing the application pages.
pub struct PageContainer {
    container: gtk::Notebook
}

impl PageContainer {

    /// Create a new page container.
    pub fn new() -> Self {
        PageContainer {
            container: Self::build_container()
        }
    }

    /// Build the container.
    fn build_container() -> gtk::Notebook {
        // Create the container instance
        let container = gtk::Notebook::new();

        // Configure the container
        container.set_hexpand(true);
        container.set_vexpand(true);
        container.set_halign(gtk::Align::Fill);
        container.set_valign(gtk::Align::Fill);

        container
    }

    /// Get the GTK widget for this page container.
    pub fn gtk_widget(&self) -> &gtk::Notebook {
        &self.container
    }
}