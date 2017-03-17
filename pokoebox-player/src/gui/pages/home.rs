use super::gtk;
use super::gtk::*;

use gui::page_builder::PageBuilder;
use gui::page_builder::PageBuilderHelper;

/// Name of the page.
const PAGE_NAME: &'static str = "Home Page";

/// Home page.
pub struct Home {
    container: gtk::Grid
}

impl Home {

    /// Constructor.
    pub fn new() -> Self {
        // Create the page instance
        let page = Home {
            container: PageBuilderHelper::create_page_container()
        };

        // Build the ui
        page.build_page();

        page
    }
}

impl PageBuilder for Home {

    fn page_name(&self) -> &'static str {
        &PAGE_NAME
    }

    fn build_page(&self) {
        // Configure the page
        self.container.set_halign(gtk::Align::Center);
        self.container.set_valign(gtk::Align::Center);

        // Create a button grid
        let buttons = gtk::Grid::new();
        buttons.set_row_spacing(8);
        buttons.set_column_spacing(8);
        buttons.set_row_homogeneous(true);
        buttons.set_column_homogeneous(true);
        buttons.set_size_request(350, 200);
        self.container.add(&buttons);

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
    }

    fn gtk_widget(&self) -> &gtk::Grid {
        &self.container
    }
}