use std::rc::Rc;
use std::sync::Arc;

use gtk::prelude::*;
use pokoebox_bluetooth::device::DeviceList;
use pokoebox_bluetooth::manager::Event;

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

        let btn_discoverable = gtk::Button::new_with_label("Connect");
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
        let (treeview, store) = build_list();
        scroll_view.add(&treeview);

        // Handle bluetooth manager events
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT_IDLE);
        core.bluetooth.events.register_callback(move |event| {
            if let Err(err) = tx.send(event) {
                error!("Failed to send bluetooth manager event to Glib: {:?}", err);
            }
        });
        let btn_discoverable = Rc::new(btn_discoverable);
        let store = Rc::new(store);
        rx.attach(None, move |event| {
            handle_bluetooth_event(event, btn_discoverable.clone(), store.clone())
        });

        // Request to emit current bluetooth state events
        if let Err(err) = core.bluetooth.emit_state() {
            error!(
                "Failed to invoke command to emit bluetooht state: {:?}",
                err
            );
        }
    }

    fn gtk_widget(&self) -> &gtk::Grid {
        &self.container
    }
}

fn handle_bluetooth_event(
    event: Event,
    btn_discoverable: Rc<gtk::Button>,
    store: Rc<gtk::ListStore>,
) -> glib::Continue {
    match event {
        Event::Discoverable(true) => {
            btn_discoverable.set_label("Discoverable...");
            btn_discoverable.set_sensitive(false);
        }
        Event::Discoverable(false) => {
            btn_discoverable.set_label("Connect");
            btn_discoverable.set_sensitive(true);
        }
        Event::DeviceConnected(_, devices)
        | Event::DeviceDisconnected(_, devices)
        | Event::Devices(devices) => {
            update_list(&store, devices);
        }
        _ => {}
    }

    glib::Continue(true)
}

#[repr(i32)]
enum Column {
    Name = 0,
    Address,
    Connected,
}

const COLUMNS: [u32; 3] = [
    Column::Name as u32,
    Column::Address as u32,
    Column::Connected as u32,
];

/// Build device list.
fn build_list() -> (gtk::TreeView, gtk::ListStore) {
    let store = gtk::ListStore::new(&[
        glib::types::Type::String,
        glib::types::Type::String,
        glib::types::Type::Bool,
    ]);

    let treeview = gtk::TreeViewBuilder::new()
        .model(&store)
        .headers_clickable(false)
        .build();

    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Device name");
        column.add_attribute(&renderer, "text", Column::Name as i32);
        column.set_expand(true);
        column.set_sizing(gtk::TreeViewColumnSizing::Autosize);
        column.set_sort_column_id(Column::Name as i32);
        treeview.append_column(&column);
    }

    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Address");
        column.add_attribute(&renderer, "text", Column::Address as i32);
        column.set_expand(true);
        column.set_sizing(gtk::TreeViewColumnSizing::Autosize);
        column.set_sort_column_id(Column::Address as i32);
        treeview.append_column(&column);
    }

    {
        let renderer = gtk::CellRendererToggle::new();
        // let model_clone = model.clone();
        // renderer.connect_toggled(move |w, path| fixed_toggled(&model_clone, w, path));
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Connected");
        column.add_attribute(&renderer, "active", Column::Connected as i32);
        treeview.append_column(&column);
    }

    (treeview, store)
}

/// Update device list with new device information.
fn update_list(store: &Rc<gtk::ListStore>, devices: DeviceList) {
    store.clear();
    for device in devices.iter() {
        store.set(
            &store.append(),
            &COLUMNS,
            &[
                &device.name.as_deref().unwrap_or("?").to_value(),
                &device.address_string().to_value(),
                &device.connected.to_value(),
            ],
        );
    }
}
