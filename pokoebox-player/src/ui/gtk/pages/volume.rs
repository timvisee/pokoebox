use std::collections::HashMap;
use std::sync::Arc;

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
        // Query list of controls
        let controls = core
            .volume
            .query_controls()
            .expect("Failed to get list of audio control");

        let scroll_window = gtk::ScrolledWindowBuilder::new().expand(true).build();

        let gbox = gtk::BoxBuilder::new()
            .expand(true)
            .orientation(gtk::Orientation::Horizontal)
            .spacing(CONTROL_SPACING)
            .margin(SPACING)
            .build();

        let mut sliders = HashMap::new();

        // Add a volume slider
        for (control, props) in controls {
            gbox.add(&build_volume_control(
                core.clone(),
                control,
                props,
                &mut sliders,
            ));
        }

        scroll_window.add(&gbox);
        self.container.add(&scroll_window);

        // Handle volume manager events
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT_IDLE);
        core.volume.mixer.events.register_callback(move |event| {
            if let Err(err) = tx.send(event) {
                error!("Failed to send volume manager event to Glib: {:?}", err);
            }
        });
        rx.attach(None, move |event| handle_volume_event(event, &sliders));
    }

    fn gtk_widget(&self) -> &gtk::Grid {
        &self.container
    }
}

fn handle_volume_event(
    event: Event,
    sliders: &HashMap<ControlHandle, (gtk::Scale, glib::signal::SignalHandlerId)>,
) -> glib::Continue {
    match event {
        // Update volume slider on volume change
        Event::Volume(control, volume) => {
            if let Some((slider, change_handler)) = sliders.get(&control) {
                slider.block_signal(change_handler);
                slider.set_value(volume as f64);
                slider.unblock_signal(change_handler);
            }
        }
        _ => {}
    }

    glib::Continue(true)
}

fn build_volume_control(
    core: Arc<Core>,
    control: ControlHandle,
    props: ControlProps,
    sliders: &mut HashMap<ControlHandle, (gtk::Scale, glib::signal::SignalHandlerId)>,
) -> gtk::Box {
    let gbox = gtk::BoxBuilder::new()
        .orientation(gtk::Orientation::Vertical)
        .spacing(SPACING)
        .width_request(50)
        .build();

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
    gbox.add(&slider);

    // Update volume on slider change
    // TODO: do not clone here, use cow in control?
    let closure_control = control.clone();
    let changed_handler = slider.connect_value_changed(move |slider| {
        if let Err(err) = core.volume.send_cmd(Cmd::SetVolume(
            closure_control.clone(),
            slider.get_value() as i64,
        )) {
            error!("Failed to set volume: {:?}", err);
        }
    });

    // Nicly format slider label
    let (range_min, range_max) = (props.range.0 as f64, props.range.1 as f64);
    slider.connect_format_value(move |_, value| {
        let diff = range_max - range_min;

        // Show yes/no
        if diff as i64 == 1 {
            if value == range_min {
                return "No".into();
            } else {
                return "Yes".into();
            }
        }

        // Show percentage
        let value = ((100f64 / diff) * value).round();
        format!("{}%", value as i64)
    });

    sliders.insert(control, (slider, changed_handler));

    // Add slider name label
    let label = gtk::LabelBuilder::new()
        .label(props.name.as_deref().unwrap_or("?"))
        .justify(gtk::Justification::Center)
        .single_line_mode(false)
        .wrap(true)
        .build();
    gbox.add(&label);

    gbox
}
