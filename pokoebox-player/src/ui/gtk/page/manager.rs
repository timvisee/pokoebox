use std::rc::Rc;
use std::sync::{Arc, Mutex};

use gtk::prelude::*;

use crate::app::Core;
use crate::pages::PageType;

use super::super::pages;
use super::Page;

/// The margin of the tab/page label
const TAB_LABEL_MARGIN: i32 = 8;

/// Struct to help manage pages in page container.
pub struct PageManager {
    /// UI container holding pages.
    container: Rc<gtk::Notebook>,

    /// Page widgets.
    pages: Mutex<Vec<Box<dyn Page>>>,
}

impl PageManager {
    pub fn new(container: Rc<gtk::Notebook>) -> Self {
        Self {
            container,
            pages: Mutex::new(Vec::new()),
        }
    }

    pub fn goto_page(&self, core: Arc<Core>, page: PageType) {
        // Goto page if loaded
        if let Some(i) = self
            .pages
            .lock()
            .expect("failed to lock pages index")
            .iter()
            .position(|p| p.page_type() == page)
        {
            self.container.set_property_page(i as i32);
            return;
        }

        // Load new page
        self.new_page(core, page);
    }

    fn new_page(&self, core: Arc<Core>, page: PageType) {
        // Initialize new page
        let page: Box<dyn Page> = match page {
            PageType::Launchpad => Box::new(pages::Launchpad::new(core)),
            PageType::Player => Box::new(pages::Player::new(core)),
            PageType::Volume => Box::new(pages::Volume::new(core)),
            PageType::Test => Box::new(pages::Test::new(core)),
        };

        // Add new page, select last tab
        self.add_page(page);
        self.container
            .set_property_page(self.container.get_children().len() as i32 - 1);
    }

    fn add_page(&self, page: Box<dyn Page>) {
        // Add the pages GTK widget to the page container
        self.container.add(page.gtk_widget());

        // Configure the tab
        self.container.set_tab_reorderable(page.gtk_widget(), true);

        // Create a tab label
        let label = gtk::Label::new(Some(page.page_name()));
        label.set_margin_start(TAB_LABEL_MARGIN);
        label.set_margin_end(TAB_LABEL_MARGIN);
        label.set_margin_top(TAB_LABEL_MARGIN);
        label.set_margin_bottom(TAB_LABEL_MARGIN);
        self.container
            .set_tab_label(page.gtk_widget(), Some(&label));

        // Update GTK notebook tabs
        self.container.show_all();

        // Add the page to the list of pages
        self.pages
            .lock()
            .expect("failed to lock pages index")
            .push(page);
    }
}
