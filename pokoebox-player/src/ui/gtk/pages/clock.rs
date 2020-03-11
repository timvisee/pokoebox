use std::sync::Arc;

use gtk::prelude::*;

use crate::app::Core;
use crate::pages::PageType;

use super::page::Helper;
use super::page::Page;

const PAGE_TYPE: PageType = PageType::Clock;
const PAGE_NAME: &str = "Clock";
const LABEL_SPACING: i32 = 16;

/// Clock.
pub struct Clock {
    container: gtk::Grid,
}

impl Clock {
    /// Constructor.
    pub fn new(core: Arc<Core>) -> Self {
        // Create the page instance
        let page = Self {
            container: Helper::create_page_container(),
        };

        // Build the page ui
        page.build_page(core);

        page
    }
}

impl Page for Clock {
    fn page_type(&self) -> PageType {
        PAGE_TYPE
    }

    fn page_name(&self) -> &'static str {
        &PAGE_NAME
    }

    fn build_page(&self, _core: Arc<Core>) {
        // Configure the page
        self.container.set_halign(gtk::Align::Center);
        self.container.set_valign(gtk::Align::Center);

        // Labels box
        let labels = gtk::Box::new(gtk::Orientation::Vertical, LABEL_SPACING);
        self.container.add(&labels);

        // Define large text attributes
        let attr_text_large = pango::AttrList::new();
        attr_text_large.insert(pango::Attribute::new_scale(5.0).unwrap());

        // Time label
        let time_label = gtk::LabelBuilder::new()
            .use_markup(true)
            .halign(gtk::Align::Center)
            .attributes(&attr_text_large)
            .build();
        labels.add(&time_label);

        // Date label
        let date_label = gtk::LabelBuilder::new().halign(gtk::Align::Center).build();
        labels.add(&date_label);

        // Update labels
        let time_tick = move || {
            let now = chrono::Local::now();
            time_label.set_label(&format!(
                "{}",
                now.format("<b>%H</b> : <b>%M</b> : <b>%S</b>")
            ));
            date_label.set_text(&format!("{}", now.format("%A %m %B, %Y")));
            gtk::prelude::Continue(true)
        };
        time_tick();
        gtk::timeout_add_seconds(1, time_tick);
    }

    fn gtk_widget(&self) -> &gtk::Grid {
        &self.container
    }
}
