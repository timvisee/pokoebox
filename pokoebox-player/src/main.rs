#[macro_use]
extern crate log;

pub mod action;
pub mod app;
pub mod error;
#[cfg(feature = "rpi")]
pub mod gpio;
pub mod gui;
pub mod perif;
pub mod result;
pub mod volume;

use crate::app::App;

fn main() {
    // Initialize logger
    simple_logger::init().unwrap();

    // Show an initial message
    info!(
        "Starting {} v{}...",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );

    // Create a new app instance
    let mut app = App::new().expect("Failed to initialize application.");

    // Start the application
    app.start().expect("Failed to start application.");

    // Start the main loop of the application
    app.main_loop();
}
