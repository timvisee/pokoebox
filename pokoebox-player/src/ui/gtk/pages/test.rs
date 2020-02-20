use std::sync::Arc;

use gtk::prelude::*;

use crate::app::Core;
use crate::pages::PageType;

use super::page::Helper;
use super::page::Page;

const PAGE_TYPE: PageType = PageType::Test;
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
        let page = Self {
            container: Helper::create_page_container(),
        };

        // Build the ui
        page.build_page(core);

        page
    }
}

impl Page for Test {
    fn page_type(&self) -> PageType {
        PAGE_TYPE
    }

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
