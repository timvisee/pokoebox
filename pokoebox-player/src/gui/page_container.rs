use std::boxed;

use super::gtk;
use super::gtk::*;

use super::page::Page;

/// The margin of the tab/page label
const TAB_LABEL_MARGIN: i32 = 8;

/// Container holding and managing the application pages.
pub struct PageContainer {
    container: gtk::Notebook,
    pages: Vec<boxed::Box<dyn Page>>,
}

impl PageContainer {
    /// Create a new page container.
    pub fn new() -> Self {
        PageContainer {
            container: Self::build_container(),
            pages: Vec::new(),
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
        container.set_tab_pos(gtk::PositionType::Bottom);
        container.set_show_border(false);

        container
    }

    /// Add the given page to the page container.
    /// The page to add must be passed to the `page` parameter.
    pub fn add_page(&mut self, page: boxed::Box<dyn Page>) {
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

        // Add the page to the list of pages
        self.pages.push(page);
    }

    /// Get the GTK widget for this page container.
    pub fn gtk_widget(&self) -> &gtk::Notebook {
        &self.container
    }
}
