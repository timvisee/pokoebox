use std::rc::Rc;
use std::sync::{mpsc, Arc};

use gtk::prelude::*;

use crate::app::Core;
use crate::pages::PageType;

use super::page::Helper;
use super::page::Page;

const PAGE_TYPE: PageType = PageType::Bluetooth;
const PAGE_NAME: &str = "Bluetooth";
const SPACING: i32 = 8;

/// Bluetooth page.
pub struct Bluetooth {
    /// Page container
    container: gtk::Grid,
}

impl Bluetooth {
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

impl Page for Bluetooth {
    fn page_type(&self) -> PageType {
        PAGE_TYPE
    }

    fn page_name(&self) -> &'static str {
        &PAGE_NAME
    }

    fn build_page(&self, core: Arc<Core>) {
        let gbox = gtk::Box::new(gtk::Orientation::Vertical, SPACING);
        self.container.add(&gbox);

        let btns = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
        btns.set_spacing(SPACING);
        btns.set_layout(gtk::ButtonBoxStyle::Center);
        gbox.add(&btns);

        let btn_power = gtk::Button::new_with_label("Power");
        btn_power.set_sensitive(false);
        btns.add(&btn_power);

        let btn_discoverable = gtk::Button::new_with_label("Discoverable");
        let core_closure = core.clone();
        btn_discoverable.connect_clicked(move |btn| {
            btn.set_sensitive(false);
            // TODO: handle result
            core_closure
                .bluetooth
                .set_discoverable(true)
                .expect("failed to set discoverable");
        });
        btns.add(&btn_discoverable);

        let scroll_view = gtk::ViewportBuilder::new().expand(true).build();
        gbox.add(&scroll_view);

        let list = gtk::ListBox::new();
        scroll_view.add(&list);

        // Handle bluetooth manager events
        // TODO: find better way to handle events
        let btn_discoverable = Rc::new(btn_discoverable);
        gtk::timeout_add_seconds(1, move || {
            let core = core.clone();
            let btn_discoverable = btn_discoverable.clone();
            handle_bluetooth_events(core, btn_discoverable)
        });
    }

    fn gtk_widget(&self) -> &gtk::Grid {
        &self.container
    }
}

fn handle_bluetooth_events(core: Arc<Core>, btn_discoverable: Rc<gtk::Button>) -> glib::Continue {
    loop {
        match core.bluetooth.events.try_recv() {
            Err(mpsc::TryRecvError::Empty) => return glib::Continue(true),
            Err(mpsc::TryRecvError::Disconnected) => return glib::Continue(false),
            Ok(event) => match event {
                pokoebox_bluetooth::Event::Discovering(true) => {
                    btn_discoverable.set_label("Discoverable...");
                    btn_discoverable.set_sensitive(false);
                }
                pokoebox_bluetooth::Event::Discovering(false) => {
                    btn_discoverable.set_label("Discoverable");
                    btn_discoverable.set_sensitive(true);
                }
                _ => {}
            },
        }
    }
}
