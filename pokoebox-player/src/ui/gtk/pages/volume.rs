use std::sync::{
    mpsc::{self, Receiver},
    Arc,
};

use gtk::{prelude::*, PositionType};
use pokoebox_audio::volume::{Cmd, ControlHandle, ControlProps, Event};

use crate::app::Core;
use crate::pages::PageType;

use super::page::Helper;
use super::page::Page;

const PAGE_TYPE: PageType = PageType::Volume;
const PAGE_NAME: &str = "Volume";
const SPACING: i32 = 8;
const CONTROL_SPACING: i32 = 32;

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

    fn build_page(&self, core: Arc<Core>) {
        // Set up page
        self.container.set_halign(gtk::Align::Center);

        // Query list of controls
        let controls = core
            .volume
            .query_controls()
            .expect("Failed to get list of audio control");

        let gbox = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(CONTROL_SPACING)
            .margin(SPACING)
            .build();

        // Add a volume slider
        for (control, props) in controls {
            let slider = build_volume_control(core.clone(), control, props);
            gbox.add(&slider);
        }

        self.container.add(&gbox);

        // Handle volume manager events
        // TODO: find better way to handle events
        let event_rx = core.volume.mixer.events.listen();
        handle_volume_events(&event_rx);
        gtk::timeout_add_seconds(1, move || handle_volume_events(&event_rx));
    }

    fn gtk_widget(&self) -> &gtk::Grid {
        &self.container
    }
}

fn handle_volume_events(event_rx: &Receiver<Event>) -> glib::Continue {
    loop {
        match event_rx.try_recv() {
            Err(mpsc::TryRecvError::Empty) => return glib::Continue(true),
            Err(mpsc::TryRecvError::Disconnected) => return glib::Continue(false),
            Ok(event) => match event {
                Event::Volume(_control, volume) => {
                    println!("Volume change event: {}", volume);
                }
                _ => {}
            },
        }
    }
}

fn build_volume_control(core: Arc<Core>, control: ControlHandle, props: ControlProps) -> gtk::Box {
    let gbox = gtk::Box::new(gtk::Orientation::Vertical, SPACING);

    let slider = gtk::Scale::new_with_range(
        gtk::Orientation::Vertical,
        props.range.0 as f64,
        props.range.1 as f64,
        1f64,
    );
    slider.set_value(props.init_value as f64);
    slider.add_mark(20f64, PositionType::Right, Some("*"));
    slider.set_vexpand(true);
    slider.set_value_pos(PositionType::Bottom);
    slider.set_inverted(true);
    let closure_control = control.clone();
    // TODO: do not clone here, use cow in control?
    slider.connect_value_changed(move |slider| {
        if let Err(err) = core.volume.send_cmd(Cmd::SetVolume(
            closure_control.clone(),
            slider.get_value() as i64,
        )) {
            error!("Failed to set volume: {:?}", err);
        }
    });
    gbox.add(&slider);

    let label = gtk::Label::new(props.name.as_deref());
    gbox.add(&label);

    gbox
}
