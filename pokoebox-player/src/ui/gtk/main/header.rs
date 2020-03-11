use std::sync::Arc;

use gtk::{prelude::*, IconSize, ReliefStyle};

use crate::action::actions::GotoPageAction;
use crate::app::Core;
use crate::pages::PageType;

/// Main UI header in the application.
pub struct Header {
    container: gtk::Box,
}

impl Header {
    /// Construct a new header.
    pub fn new(core: Arc<Core>) -> Self {
        Header {
            container: Self::build_ui(core),
        }
    }

    /// Build the header.
    fn build_ui(core: Arc<Core>) -> gtk::Box {
        // Create a container instance
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        // Configure the header
        container.set_hexpand(true);
        container.set_vexpand(false);
        container.set_halign(gtk::Align::Fill);
        container.set_border_width(8);

        // Build and add controls
        Self::build_ui_controls(core, &container);

        container
    }

    /// Build and add header controls.
    fn build_ui_controls(core: Arc<Core>, container: &gtk::Box) {
        // Create header label
        let header_btn = gtk::ButtonBuilder::new()
            .relief(gtk::ReliefStyle::None)
            .build();
        header_btn.set_focus_on_click(false);
        let header_lbl = gtk::LabelBuilder::new()
            .label("<b>PokoeBox</b>")
            .use_markup(true)
            .build();
        header_btn.add(&header_lbl);
        container.set_center_widget(Some(&header_btn));

        // Go to about page on click
        let closure_core = core.clone();
        header_btn.connect_clicked(move |_| {
            // TODO: handle result
            let _ = closure_core
                .actions
                .invoke(GotoPageAction::new(PageType::About), closure_core.clone());
        });

        // Create a home button
        let home_button =
            gtk::Button::new_from_icon_name(Some("view-grid"), IconSize::LargeToolbar);
        home_button.set_relief(ReliefStyle::None);
        home_button.set_focus_on_click(false);
        let closure_core = core.clone();
        home_button.connect_clicked(move |_| {
            // TODO: handle result
            let _ = closure_core
                .actions
                .invoke(GotoPageAction::new_home(), closure_core.clone());
        });
        container.pack_start(&home_button, false, false, 10);

        // Create a volume button
        let volume = gtk::VolumeButton::new();
        container.pack_end(&volume, false, false, 10);

        // Create a time label
        let time_label = gtk::ButtonBuilder::new()
            .relief(gtk::ReliefStyle::None)
            .build();
        time_label.set_focus_on_click(false);
        container.pack_end(&time_label, false, false, 0);

        // Go to clock page on click
        let closure_core = core.clone();
        time_label.connect_clicked(move |_| {
            // TODO: handle result
            let _ = closure_core
                .actions
                .invoke(GotoPageAction::new(PageType::Clock), closure_core.clone());
        });

        // Update time label
        let time_tick = move || {
            // TODO: show seconds in header?
            time_label.set_label(&format!("{}", chrono::Local::now().format("%H:%M:%S")));
            gtk::prelude::Continue(true)
        };
        time_tick();
        gtk::timeout_add_seconds(1, time_tick);

        // Create a power label
        #[cfg(feature = "rpi")]
        {
            let power_label = gtk::ButtonBuilder::new()
                .label("Power: ?")
                .relief(gtk::ReliefStyle::None)
                .build();
            power_label.set_focus_on_click(false);
            container.pack_end(&power_label, false, false, 0);

            // Go to power page on click
            let closure_core = core.clone();
            power_label.connect_clicked(move |_| {
                // TODO: handle result
                let _ = closure_core
                    .actions
                    .invoke(GotoPageAction::new(PageType::Power), closure_core.clone());
            });

            // Update label on power events
            let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT_IDLE);
            core.power.events.register_callback(move |event| {
                if let Err(err) = tx.send(event) {
                    error!("Failed to send power event to Glib: {:?}", err);
                }
            });
            rx.attach(None, move |event| {
                let pokoebox_rpi::power::Event::Power(_, voltage, _) = event;
                power_label.set_label(&format!("{:.1} V", voltage,));

                gtk::prelude::Continue(true)
            });

            // Poll power interface
            // TODO: do this in power manager, not here
            let power_poll = move || {
                // Poll power interface
                if let Err(err) = core.power.send_cmd(pokoebox_rpi::power::Cmd::Poll) {
                    error!("Failed to poll power interface: {:?}", err);
                }
                gtk::prelude::Continue(true)
            };
            power_poll();
            gtk::timeout_add_seconds(2, power_poll);
        }
    }

    /// Get the GTK widget for this header.
    pub fn gtk_widget(&self) -> &gtk::Box {
        &self.container
    }
}
