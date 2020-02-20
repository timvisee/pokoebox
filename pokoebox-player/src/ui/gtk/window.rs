use gtk::{prelude::*, Inhibit};

use super::main::App;

const TITLE: &str = "PokoeBox Player";
const WIDTH: i32 = 1024;
const HEIGHT: i32 = 600;

/// The main window.
pub struct Window {
    window: gtk::Window,
}

impl Window {
    /// Create new window.
    pub fn new() -> Self {
        Self {
            window: Self::build_window(),
        }
    }

    /// Build the window UI.
    fn build_window() -> gtk::Window {
        // Create a window instance
        let window = gtk::Window::new(gtk::WindowType::Toplevel);

        // Configure the window
        window.set_title(TITLE);
        window.set_border_width(0);
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(WIDTH, HEIGHT);

        // Connect the window delete event
        window.connect_delete_event(|_, _| {
            // Close the application
            gtk::main_quit();

            // Leave window destruction to GTK
            Inhibit(false)
        });

        window
    }

    /// Set the app UI in the window.
    pub fn set_app(&self, app: &App) {
        // TODO: Should we remove the current child(ren) to replace them?

        self.window.add(&app.grid);
    }

    /// Show the window.
    pub fn show(&self) {
        self.window.show_all();
    }
}
