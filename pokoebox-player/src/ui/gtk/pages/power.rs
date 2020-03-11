use std::sync::Arc;

use gtk::prelude::*;

use crate::app::Core;
use crate::pages::PageType;
use crate::util;

use super::page::Helper;
use super::page::Page;

const PAGE_TYPE: PageType = PageType::Power;
const PAGE_NAME: &str = "Power";
const LABEL_SPACING: u32 = 16;

/// Power.
pub struct Power {
    container: gtk::Grid,
}

impl Power {
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

impl Page for Power {
    fn page_type(&self) -> PageType {
        PAGE_TYPE
    }

    fn page_name(&self) -> &'static str {
        &PAGE_NAME
    }

    fn build_page(&self, core: Arc<Core>) {
        // Configure the page
        self.container.set_halign(gtk::Align::Center);
        self.container.set_valign(gtk::Align::Center);

        // Create a label grid
        let grid = gtk::Grid::new();
        grid.set_row_spacing(LABEL_SPACING);
        grid.set_column_spacing(LABEL_SPACING);
        self.container.add(&grid);

        // Page header
        let header = gtk::LabelBuilder::new()
            .label("<b>Power stats:</b>")
            .use_markup(true)
            .halign(gtk::Align::Start)
            .build();
        grid.attach(&header, 0, 0, 2, 1);

        // State header
        let header_state = gtk::LabelBuilder::new()
            .label("State:")
            .halign(gtk::Align::Start)
            .build();
        grid.attach(&header_state, 0, 1, 1, 1);

        // State label
        let label_state = gtk::LabelBuilder::new()
            .label("<i>Unknown</i>")
            .use_markup(true)
            .halign(gtk::Align::Start)
            .build();
        grid.attach(&label_state, 1, 1, 1, 1);

        // Voltage header
        let header_voltage = gtk::LabelBuilder::new()
            .label("Voltage:")
            .halign(gtk::Align::Start)
            .build();
        grid.attach(&header_voltage, 0, 2, 1, 1);

        // Voltage label
        let label_voltage = gtk::LabelBuilder::new()
            .label("<i>Unknown</i>")
            .use_markup(true)
            .halign(gtk::Align::Start)
            .build();
        grid.attach(&label_voltage, 1, 2, 1, 1);

        // Power header
        let header_power = gtk::LabelBuilder::new()
            .label("Power:")
            .halign(gtk::Align::Start)
            .build();
        grid.attach(&header_power, 0, 3, 1, 1);

        // Power label
        let label_power = gtk::LabelBuilder::new()
            .label("<i>Unknown</i>")
            .use_markup(true)
            .halign(gtk::Align::Start)
            .build();
        grid.attach(&label_power, 1, 3, 1, 1);

        // Current header
        let header_current = gtk::LabelBuilder::new()
            .label("Current:")
            .halign(gtk::Align::Start)
            .build();
        grid.attach(&header_current, 0, 4, 1, 1);

        // Current label
        let label_current = gtk::LabelBuilder::new()
            .label("<i>Unknown</i>")
            .use_markup(true)
            .halign(gtk::Align::Start)
            .build();
        grid.attach(&label_current, 1, 4, 1, 1);

        // Update labels on power events
        #[cfg(feature = "rpi")]
        {
            let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT_IDLE);
            core.power.events.register_callback(move |event| {
                if let Err(err) = tx.send(event) {
                    error!("Failed to send power event to Glib: {:?}", err);
                }
            });
            rx.attach(None, move |event| {
                let pokoebox_rpi::power::Event::Power(current, voltage, power) = event;
                label_state.set_label(match current {
                    x if x > 0.0 => "Discharging",
                    x if x < 0.0 => "Charging",
                    _ => "<i>Idle</i>",
                });
                label_voltage.set_label(&format!("{} V", util::format_num_sig(voltage, 4)));
                label_power.set_label(&format!("{} W", util::format_num_sig(power, 4)));
                label_current.set_label(&format!("{} A", util::format_num_sig(current.abs(), 4)));

                gtk::prelude::Continue(true)
            });
        }
    }

    fn gtk_widget(&self) -> &gtk::Grid {
        &self.container
    }
}
