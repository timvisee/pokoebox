use std::sync::Arc;

use gtk::{self, prelude::*};

use crate::app::Core;

use super::page::Helper;
use super::page::Page;

/// Name of the page.
const PAGE_NAME: &str = "Test";

/// Test page.
pub struct Test {
    /// Page container
    container: gtk::Grid,
}

impl Test {
    /// Constructor.
    pub fn new(core: Arc<Core>) -> Self {
        // Create the page instance
        let page = Test {
            container: Helper::create_page_container(),
        };

        // Build the ui
        page.build_page(core);

        page
    }
}

impl Page for Test {
    fn page_name(&self) -> &'static str {
        &PAGE_NAME
    }

    fn build_page(&self, _core: Arc<Core>) {
        // Add a test button
        let button = gtk::Button::new_with_label("Test button");
        self.container.add(&button);
    }

    fn gtk_widget(&self) -> &gtk::Grid {
        &self.container
    }
}
