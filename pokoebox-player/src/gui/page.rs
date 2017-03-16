use super::gtk;
use super::gtk::*;

pub struct Page {
    name: &'static str,
    page: gtk::Grid
}

impl Page {

    /// Construct a new page.
    /// A name for the page should be provided to the `name` parameter.
    pub fn new(name: &'static str) -> Self {
        Page {
            name: name,
            page: Self::build_page()
        }
    }

    /// Build the GTK page widget.
    fn build_page() -> gtk::Grid {
        // Construct the page
        let page = gtk::Grid::new();

        // Configure the page
        page.set_hexpand(true);
        page.set_vexpand(true);
        page.set_border_width(8);

        let button = gtk::Button::new_with_label("SOMEOISJFOIJD TEST");
        page.add(&button);

        page
    }

    /// Get the name of the page.
    pub fn name(&self) -> &'static str {
        &self.name
    }

    /// Get the GTK widget for this page.
    pub fn gtk_widget(&self) -> &gtk::Grid {
        &self.page
    }
}