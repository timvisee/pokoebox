use super::gtk;
use super::gtk::*;

use gui::page::Page;
use gui::page::Helper;

/// Name of the page.
const PAGE_NAME: &'static str = "Test";

/// Test page.
pub struct Test {
    /// Page container
    container: gtk::Grid
}

impl Test {

    /// Constructor.
    pub fn new() -> Self {
        // Create the page instance
        let page = Test {
            container: Helper::create_page_container()
        };

        // Build the ui
        page.build_page();

        page
    }
}

impl Page for Test {

    fn page_name(&self) -> &'static str {
        &PAGE_NAME
    }

    fn build_page(&self) {
        // Add a test button
        let button = gtk::Button::new_with_label("Test button");
        self.container.add(&button);
    }

    fn gtk_widget(&self) -> &gtk::Grid {
        &self.container
    }
}