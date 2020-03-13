use std::sync::{Arc, Mutex, MutexGuard};

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

    /// The list box.
    list: gtk::ListBox,

    /// List of sources
    sources: Mutex<Vec<Source>>,
}

impl SourceList {
    fn build() -> Self {
        // Base container, scrollable
        let container = gtk::ScrolledWindowBuilder::new()
            .expand(true)
            .shadow_type(gtk::ShadowType::EtchedIn)
            .build();

        // Source list
        let list = gtk::ListBoxBuilder::new()
            // .spacing(4)
            // .orientation(gtk::Orientation::Vertical)
            .expand(true)
            // .halign(gtk::Align::Start)
            // .valign(gtk::Align::Start)
            .border_width(40)
            .build();
        container.add(&list);

        Self {
            container,
            list,
            sources: Mutex::new(Vec::new()),
        }
    }

    pub fn update_sources(&self, states: Vec<(Handle, State)>) {
        // Lock sources list
        let mut sources = self.sources.lock().expect("Failed to lock sources list");

        // Add new sources, remove old sources
        // TODO: update changed sources
        states.iter().for_each(|(handle, _state)| {
            if !sources.iter().any(|s| &s.handle == handle) {
                self.add_source(&mut sources, *handle);
            }
        });

        sources
            .iter()
            .filter(|source| !states.iter().any(|(handle, _)| &source.handle == handle))
            .map(|source| source.handle)
            .collect::<Vec<Handle>>()
            .iter()
            .for_each(|h| {
                if !states.iter().any(|(handle, _)| h == handle) {
                    self.remove_source(&mut sources, *h);
                }
            });

        // Update view
        self.list.show_all();
    }

    fn add_source<'a>(&self, sources: &mut MutexGuard<'a, Vec<Source>>, handle: Handle) {
        let source = Source::build(handle);
        self.list.insert(&source.row, -1);
        sources.push(source);
    }

    fn remove_source<'a>(&self, sources: &mut MutexGuard<'a, Vec<Source>>, handle: Handle) {
        let source = match sources.iter().find(|s| s.handle == handle) {
            Some(source) => source,
            None => return,
        };

        self.list.remove(&source.container);
    }
}

struct Source {
    pub row: gtk::ListBoxRow,
    pub container: gtk::Box,
    pub handle: Handle,
}

impl Source {
    fn build(handle: Handle) -> Self {
        // Build container
        let container = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            // .margin(4)
            .hexpand(true)
            .halign(gtk::Align::Start)
            .spacing(4)
            .margin(4)
            .build();

        // Add labels
        let source_lbl = gtk::LabelBuilder::new()
            .label(&format!("<b>Source: {:?}</b>", handle))
            .use_markup(true)
            .build();
        container.add(&source_lbl);

        let track_lbl = gtk::Label::new(Some("Track"));
        container.add(&track_lbl);

        let row = gtk::ListBoxRowBuilder::new().child(&container).build();

        Self {
            row,
            container,
            handle,
        }
    }
}
