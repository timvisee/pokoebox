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
        page.set_halign(gtk::Align::Center);
        page.set_valign(gtk::Align::Center);

        // Create a button grid
        let buttons = gtk::Grid::new();
        buttons.set_row_spacing(8);
        buttons.set_column_spacing(8);
        buttons.set_row_homogeneous(true);
        buttons.set_column_homogeneous(true);
        buttons.set_size_request(350, 200);
        page.add(&buttons);

        // Add some buttons
        let button1 = gtk::Button::new_with_label("Play");
        buttons.attach(&button1, 0, 0, 1, 1);
        let button2 = gtk::Button::new_with_label("Source");
        buttons.attach(&button2, 1, 0, 1, 1);
        let button3 = gtk::Button::new_with_label("Spotify");
        buttons.attach(&button3, 2, 0, 1, 1);
        let button4 = gtk::Button::new_with_label("Connect");
        buttons.attach(&button4, 0, 1, 1, 1);
        let button5 = gtk::Button::new_with_label("Devices");
        buttons.attach(&button5, 1, 1, 1, 1);
        let button6 = gtk::Button::new_with_label("Settings");
        buttons.attach(&button6, 2, 1, 1, 1);

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