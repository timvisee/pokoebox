use std::sync::Arc;

use gtk::{prelude::*, PositionType};

use crate::app::Core;
use crate::pages::PageType;

use super::page::Helper;
use super::page::Page;

const PAGE_TYPE: PageType = PageType::Volume;
const PAGE_NAME: &str = "Volume";

/// Volume page.
pub struct Volume {
    /// Page container
    container: gtk::Grid,
}

impl Volume {
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

impl Page for Volume {
    fn page_type(&self) -> PageType {
        PAGE_TYPE
    }

    fn page_name(&self) -> &'static str {
        &PAGE_NAME
    }

    fn build_page(&self, _core: Arc<Core>) {
        // Add a volume slider
        let slider = gtk::Scale::new_with_range(gtk::Orientation::Vertical, 0f64, 100f64, 0.1f64);
        slider.add_mark(50f64, PositionType::Right, Some("M"));
        slider.set_vexpand(true);
        slider.set_value_pos(PositionType::Right);
        slider.set_inverted(true);
        self.container.add(&slider);
    }

    fn gtk_widget(&self) -> &gtk::Grid {
        &self.container
    }
}
