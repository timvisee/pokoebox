use super::gtk;
use super::gtk::*;

/// Main UI header in the application.
pub struct Header {
    container: gtk::Box
}

impl Header {

    /// Construct a new header.
    pub fn new() -> Self {
        Header {
            container: Self::build_container()
        }
    }

    /// Build the header container.
    fn build_container() -> gtk::Box {
        // Create a container instance
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        // Configure the header
        container.set_hexpand(true);
        container.set_vexpand(false);
        container.set_halign(gtk::Align::Fill);
        container.set_border_width(8);

        // Build the container controls
        Self::build_container_controls(&container);

        container
    }

    /// Build and add container controls to the container
    fn build_container_controls(container: &gtk::Box) {
        // Create a home button
        let home_button = gtk::Button::new();
        let home_image = gtk::Image::new_from_icon_name(
            "go-home",
            IconSize::LargeToolbar.into()
        );
        home_button.add(&home_image);
        home_button.set_relief(ReliefStyle::None);
        home_button.set_focus_on_click(false);
        container.pack_start(&home_button, false, false, 0);

        // Create a temperature label
        let temp_label = gtk::Label::new("Temp: 56°C");
        container.pack_end(&temp_label, false, false, 0);

        // Create a volume button
        let volume = gtk::VolumeButton::new();
        container.pack_end(&volume, false, false, 20);
    }

    /// Get the GTK widget for this header.
    pub fn gtk_widget(&self) -> &gtk::Box {
        &self.container
    }
}
