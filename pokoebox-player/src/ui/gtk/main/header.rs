use std::sync::Arc;

use gtk::{prelude::*, IconSize, ReliefStyle};

use crate::action::actions::GotoPageAction;
use crate::app::Core;

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
        // Create a home button
        let home_button =
            gtk::Button::new_from_icon_name(Some("view-grid"), IconSize::LargeToolbar);
        home_button.set_relief(ReliefStyle::None);
        home_button.set_focus_on_click(false);
        home_button.connect_clicked(move |_| {
            // TODO: handle result
            let _ = core
                .actions
                .invoke(GotoPageAction::new_home(), core.clone());
        });
        container.pack_start(&home_button, false, false, 10);

        // Create a volume button
        let volume = gtk::VolumeButton::new();
        container.pack_end(&volume, false, false, 10);

        // Create a time label
        let time_label = gtk::Label::new(None);
        container.pack_end(&time_label, false, false, 10);
        let time_tick = move || {
            time_label.set_text(&format!("{}", chrono::Local::now().format("%H:%M:%S")));
            gtk::prelude::Continue(true)
        };
        time_tick();
        gtk::timeout_add_seconds(1, time_tick);

        // Create a charge label
        let charge_label = gtk::Label::new(None);
        container.pack_end(&charge_label, false, false, 10);
        let charge_tick = move || {
            charge_label.set_text("20.0V");
            gtk::prelude::Continue(true)
        };
        charge_tick();
        gtk::timeout_add_seconds(5, charge_tick);

        // Create header label
        let label_header = gtk::LabelBuilder::new()
            .label("<b>PokoeBox</b>")
            .use_markup(true)
            .build();
        container.set_center_widget(Some(&label_header));
    }

    /// Get the GTK widget for this header.
    pub fn gtk_widget(&self) -> &gtk::Box {
        &self.container
    }
}
