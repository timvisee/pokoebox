use std::sync::Arc;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use pokoebox_media::mpris::Cmd;
use pokoebox_media::player::{Handle, State};

use crate::app::Core;
use crate::pages::PageType;

use super::page::Helper;
use super::page::Page;

const PAGE_TYPE: PageType = PageType::Player;
const PAGE_NAME: &str = "Player";
const BUTTON_SPACING: u32 = 16;
const BUTTON_SIZE: (i32, i32) = (80, 80);

/// Player page.
pub struct Player {
    /// Page container
    container: gtk::Grid,
}

impl Player {
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

    fn build_controls_container(core: &Arc<Core>) -> gtk::Box {
        // Controls container
        let controls = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .spacing(BUTTON_SPACING as i32)
            .expand(true)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .build();

        let player_label = gtk::Label::new(Some("Sources: ?"));
        controls.add(&player_label);

        // Add source label
        let source_label = gtk::Label::new(Some("Source: ?"));
        controls.add(&source_label);

        // Add track info label
        let track_info = gtk::Label::new(Some("Track: ?"));
        controls.add(&track_info);

        // Create a button grid
        let btns = gtk::Grid::new();
        btns.set_row_spacing(BUTTON_SPACING);
        btns.set_column_spacing(BUTTON_SPACING);
        btns.set_row_homogeneous(true);
        btns.set_column_homogeneous(true);
        controls.add(&btns);

        let btn_prev = gtk::Button::new_from_icon_name(
            Some("media-skip-backward"),
            gtk::IconSize::LargeToolbar,
        );
        btn_prev.set_size_request(BUTTON_SIZE.0, BUTTON_SIZE.1);
        btn_prev.connect_clicked(clone!(@weak core => move |_| {
            core
                .mpris
                .send_cmd(Cmd::Previous)
                .expect("failed to send signal");
        }));
        btns.add(&btn_prev);

        let btn_play = gtk::Button::new_from_icon_name(
            Some("media-playback-start"),
            gtk::IconSize::LargeToolbar,
        );
        btn_play.set_size_request(BUTTON_SIZE.0, BUTTON_SIZE.1);
        btn_play.connect_clicked(clone!(@weak core => move |_| {
            core.mpris
                .send_cmd(Cmd::Play)
                .expect("failed to send signal");
        }));
        btns.add(&btn_play);

        let btn_stop = gtk::Button::new_from_icon_name(
            Some("media-playback-stop"),
            gtk::IconSize::LargeToolbar,
        );
        btn_stop.set_size_request(BUTTON_SIZE.0, BUTTON_SIZE.1);
        btn_stop.connect_clicked(clone!(@weak core => move |_| {
            core.mpris
                .send_cmd(Cmd::Pause)
                .expect("failed to send signal");
        }));
        btns.add(&btn_stop);

        let btn_fwd = gtk::Button::new_from_icon_name(
            Some("media-skip-forward"),
            gtk::IconSize::LargeToolbar,
        );
        btn_fwd.set_size_request(BUTTON_SIZE.0, BUTTON_SIZE.1);
        btn_fwd.connect_clicked(clone!(@weak core => move |_| {
            core.mpris
                .send_cmd(Cmd::Next)
                .expect("failed to send signal");
        }));
        btns.add(&btn_fwd);

        // Handle MPRIS manager events
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT_IDLE);
        core.mpris.events().register_callback(move |event| {
            if let Err(err) = tx.send(event) {
                error!("Failed to send MPRIS manager event to Glib: {:?}", err);
            }
        });
        rx.attach(None, move |event| {
            use pokoebox_media::mpris::Event;

            match event {
                Event::Players(players) => {
                    if !players.is_empty() {
                        source_label.set_label(&format!("Source: {}", players[0].name));
                    } else {
                        source_label.set_label("Source: ?");
                    }
                }
                Event::TrackInfo(info) => {
                    track_info.set_label(&info);
                }
                _ => {}
            }
            glib::Continue(true)
        });

        // Request to find new MPRIS players.
        if let Err(err) = core.mpris.find_players() {
            error!(
                "Failed to invoke command to find available MPRIS players: {:?}",
                err
            );
        }

        // Handle player events
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT_IDLE);
        core.player.events.register_callback(move |event| {
            if let Err(err) = tx.send(event) {
                error!("Failed to send player event to Glib: {:?}", err);
            }
        });
        rx.attach(None, move |event| {
            use pokoebox_media::player::player::Event;
            use pokoebox_media::player::source::Event as SourceEvent;

            match event {
                Event::Source(event) => match event {
                    SourceEvent::States(states) => {
                        let names = states
                            .iter()
                            .map(|s| s.1.name.to_owned())
                            .collect::<Vec<_>>()
                            .join(", ");
                        player_label.set_label(&format!("Sources: {}", names));
                    }
                    SourceEvent::Add(_, _) | SourceEvent::Remove(_) => {}
                },
            }

            glib::Continue(true)
        });

        // Emit last source states
        match core.player.sources.lock() {
            Ok(sources) => sources.emit_states(),
            Err(err) => error!(
                "Failed to request player to emit states event, ignoring: {:?}",
                err
            ),
        }

        controls
    }

    fn build_sources_container(core: &Arc<Core>) -> Arc<SourceList> {
        // Create source list
        let source_list = Arc::new(SourceList::build());

        // Handle player events
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT_IDLE);
        core.player.events.register_callback(move |event| {
            if let Err(err) = tx.send(event) {
                error!("Failed to send player event to Glib: {:?}", err);
            }
        });
        rx.attach(
            None,
            // TODO: use @weak?
            clone!(@strong source_list => @default-return glib::Continue(false), move |event| {
                use pokoebox_media::player::player::Event;
                use pokoebox_media::player::source::Event as SourceEvent;

                match event {
                    Event::Source(event) => match event {
                        SourceEvent::States(states) => source_list.update_sources(states),
                        SourceEvent::Add(_, _) | SourceEvent::Remove(_) => {}
                    },
                }

                glib::Continue(true)
            }),
        );

        // Emit last source states
        match core.player.sources.lock() {
            Ok(sources) => sources.emit_states(),
            Err(err) => error!(
                "Failed to request player to emit states event, ignoring: {:?}",
                err
            ),
        }

        source_list
    }
}

impl Page for Player {
    fn page_type(&self) -> PageType {
        PAGE_TYPE
    }

    fn page_name(&self) -> &'static str {
        &PAGE_NAME
    }

    fn build_page(&self, core: Arc<Core>) {
        // Main container.
        let container = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(BUTTON_SPACING as i32)
            .expand(true)
            .build();
        self.container.add(&container);

        // Build containers
        container.add(&Self::build_controls_container(&core));
        container.add(&Self::build_sources_container(&core).container);
    }

    fn gtk_widget(&self) -> &gtk::Grid {
        &self.container
    }
}

struct SourceList {
    /// The container.
    pub container: gtk::ScrolledWindow,

    model: gio::ListStore,
    // /// The list box.
    // list: gtk::ListBox,

    // /// List of sources
    // sources: Mutex<Vec<Source>>,
}

impl SourceList {
    fn build() -> Self {
        // Base container, scrollable
        let container = gtk::ScrolledWindowBuilder::new()
            .expand(true)
            .shadow_type(gtk::ShadowType::EtchedIn)
            .build();

        let model = gio::ListStore::new(RowData::static_type());

        // Source list
        let list = gtk::ListBoxBuilder::new()
            .expand(true)
            .border_width(40)
            .selection_mode(gtk::SelectionMode::None)
            .build();

        list.bind_model(Some(&model), |item| {
            let item = item
                .downcast_ref::<RowData>()
                .expect("Row data is of wrong type");

            // TODO: dummy handle, replace with proper data
            let handle = Handle::from(1);
            let source = Source::build(handle, item);

            source.row.show_all();
            source.row.upcast::<gtk::Widget>()
        });

        container.add(&list);

        Self { container, model }
    }

    /// Update the list of sources.
    pub fn update_sources(&self, states: Vec<(Handle, State)>) {
        self.model.remove_all();
        for (handle, state) in states {
            self.model
                .append(&RowData::new(handle, &state.name, handle.0 as u32));
        }
    }
}

struct Source {
    pub row: gtk::ListBoxRow,
    pub container: gtk::Box,
    pub handle: Handle,
}

impl Source {
    fn build(handle: Handle, item: &RowData) -> Self {
        // Build container
        let container = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .halign(gtk::Align::Start)
            .spacing(8)
            .margin(4)
            .build();

        let labels = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .spacing(4)
            .build();
        container.add(&labels);

        // Add labels
        let source_lbl = gtk::LabelBuilder::new()
            // .label(&format!("<b>Source: {:?}</b>", handle))
            .use_markup(true)
            .build();
        item.bind_property("name", &source_lbl, "label")
            .flags(
                // TODO: update flags
                glib::BindingFlags::DEFAULT
                    | glib::BindingFlags::SYNC_CREATE
                    | glib::BindingFlags::BIDIRECTIONAL,
            )
            .build();
        labels.add(&source_lbl);

        let track_lbl = gtk::Label::new(Some("TODO: track info"));
        labels.add(&track_lbl);

        let btn_play = gtk::Button::new_from_icon_name(
            Some("media-playback-start"),
            gtk::IconSize::LargeToolbar,
        );
        btn_play.set_sensitive(false);
        container.add(&btn_play);
        let btn_stop = gtk::Button::new_from_icon_name(
            Some("media-playback-stop"),
            gtk::IconSize::LargeToolbar,
        );
        btn_stop.set_sensitive(false);
        container.add(&btn_stop);

        let row = gtk::ListBoxRowBuilder::new()
            .child(&container)
            .can_focus(false)
            .build();

        Self {
            row,
            container,
            handle,
        }
    }
}

use row_data::RowData;

// Our GObject subclass for carrying a name and count for the ListBox model
//
// Both name and count are stored in a RefCell to allow for interior mutability
// and are exposed via normal GObject properties. This allows us to use property
// bindings below to bind the values with what widgets display in the UI
mod row_data {
    use super::*;

    use glib::subclass;
    use glib::subclass::prelude::*;
    use glib::translate::*;

    // Implementation sub-module of the GObject
    mod imp {
        use super::*;
        use std::cell::RefCell;

        // The actual data structure that stores our values. This is not accessible
        // directly from the outside.
        pub struct RowData {
            handle: RefCell<u64>,
            name: RefCell<Option<String>>,
            count: RefCell<u32>,
        }

        // GObject property definitions for our two values
        static PROPERTIES: [subclass::Property; 3] = [
            subclass::Property("handle", |name| {
                glib::ParamSpec::uint64(
                    name,
                    "Handle",
                    "Handle",
                    0,
                    std::u64::MAX,
                    0,
                    glib::ParamFlags::READWRITE,
                )
            }),
            subclass::Property("name", |name| {
                glib::ParamSpec::string(
                    name,
                    "Name",
                    "Name",
                    None, // Default value
                    glib::ParamFlags::READWRITE,
                )
            }),
            subclass::Property("count", |name| {
                glib::ParamSpec::uint(
                    name,
                    "Count",
                    "Count",
                    0,
                    100,
                    0, // Allowed range and default value
                    glib::ParamFlags::READWRITE,
                )
            }),
        ];

        // Basic declaration of our type for the GObject type system
        impl ObjectSubclass for RowData {
            const NAME: &'static str = "RowData";
            type ParentType = glib::Object;
            type Instance = subclass::simple::InstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            glib_object_subclass!();

            // Called exactly once before the first instantiation of an instance. This
            // sets up any type-specific things, in this specific case it installs the
            // properties so that GObject knows about their existence and they can be
            // used on instances of our type
            fn class_init(klass: &mut Self::Class) {
                klass.install_properties(&PROPERTIES);
            }

            // Called once at the very beginning of instantiation of each instance and
            // creates the data structure that contains all our state
            fn new() -> Self {
                Self {
                    handle: RefCell::new(0),
                    name: RefCell::new(None),
                    count: RefCell::new(0),
                }
            }
        }

        // The ObjectImpl trait provides the setters/getters for GObject properties.
        // Here we need to provide the values that are internally stored back to the
        // caller, or store whatever new value the caller is providing.
        //
        // This maps between the GObject properties and our internal storage of the
        // corresponding values of the properties.
        impl ObjectImpl for RowData {
            glib_object_impl!();

            fn set_property(&self, _obj: &glib::Object, id: usize, value: &glib::Value) {
                let prop = &PROPERTIES[id];

                match *prop {
                    subclass::Property("handle", ..) => {
                        let handle = value
                            .get_some()
                            .expect("type conformity checked by `Object::set_property`");
                        self.handle.replace(handle);
                    }
                    subclass::Property("name", ..) => {
                        let name = value
                            .get()
                            .expect("type conformity checked by `Object::set_property`");
                        self.name.replace(name);
                    }
                    subclass::Property("count", ..) => {
                        let count = value
                            .get_some()
                            .expect("type conformity checked by `Object::set_property`");
                        self.count.replace(count);
                    }
                    _ => unimplemented!(),
                }
            }

            fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
                let prop = &PROPERTIES[id];

                match *prop {
                    subclass::Property("handle", ..) => Ok(self.handle.borrow().to_value()),
                    subclass::Property("name", ..) => Ok(self.name.borrow().to_value()),
                    subclass::Property("count", ..) => Ok(self.count.borrow().to_value()),
                    _ => unimplemented!(),
                }
            }
        }
    }

    // Public part of the RowData type. This behaves like a normal gtk-rs-style GObject
    // binding
    glib_wrapper! {
        pub struct RowData(Object<subclass::simple::InstanceStruct<imp::RowData>, subclass::simple::ClassStruct<imp::RowData>, RowDataClass>);

        match fn {
            get_type => || imp::RowData::get_type().to_glib(),
        }
    }

    // Constructor for new instances. This simply calls glib::Object::new() with
    // initial values for our two properties and then returns the new instance
    impl RowData {
        pub fn new(handle: Handle, name: &str, count: u32) -> RowData {
            glib::Object::new(
                Self::static_type(),
                &[
                    ("handle", &(handle.0 as u64)),
                    ("name", &name),
                    ("count", &count),
                ],
            )
            .expect("Failed to create row data")
            .downcast()
            .expect("Created row data is of wrong type")
        }
    }
}
