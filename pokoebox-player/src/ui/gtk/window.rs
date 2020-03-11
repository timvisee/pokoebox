use std::sync::Arc;

use glib::clone;
use gtk::{prelude::*, Inhibit};

use super::main::App;
use crate::app::Core;
use crate::message::Message;

const TITLE: &str = "PokoeBox Player";
const WIDTH: i32 = 1024;
const HEIGHT: i32 = 600;

/// The main window.
pub struct Window {
    window: gtk::Window,
}

impl Window {
    /// Create new window.
    pub fn new(core: Arc<Core>) -> Self {
        // Build window, set up message handler
        let window = Self {
            window: Self::build_window(),
        };
        window.setup_message_handler(core);
        window
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

    /// Set up the message pipe handler.
    ///
    /// This shows visual dialogs for each message.
    fn setup_message_handler(&self, core: Arc<Core>) {
        // Handle message pipe packets
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT_IDLE);
        core.messages.register_callback(move |msg| {
            if let Err(err) = tx.send(msg) {
                error!("Failed to send message to Glib: {:?}", err);
            }
        });
        rx.attach(
            None,
            clone!(@strong self.window as window => move |msg| {
                // Build message dialog
                let Message::Error(msg) = msg;
                let dialog = gtk::MessageDialog::new(
                    Some(&window),
                    gtk::DialogFlags::MODAL,
                    gtk::MessageType::Error,
                    gtk::ButtonsType::Close,
                    &msg,
                );

                // Show dialog, destroy on close
                dialog.run();
                dialog.destroy();

                glib::Continue(true)
            }),
        );
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
