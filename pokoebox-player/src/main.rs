#[macro_use]
extern crate log;
extern crate simple_logger;

pub mod action;
pub mod app;
pub mod error;
#[cfg(feature = "rpi")]
pub mod gpio;
pub mod gui;
pub mod manifest;
pub mod perif;
pub mod result;
pub mod volume;

use app::App;

fn main() {
    // Initialize logger
    simple_logger::init().unwrap();

    // Show an initial message
    info!(
        "Starting {} v{}...",
        manifest::APP_NAME,
        manifest::APP_VERSION_NAME
    );
    info!("Developed by {}.", manifest::APP_ABOUT);

    // Create a new app instance
    let mut app = App::new().expect("Failed to initialize application.");

    // Start the application
    app.start().expect("Failed to start application.");

    // Start the main loop of the application
    app.main_loop();
}
