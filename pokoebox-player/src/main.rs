#[macro_use]
extern crate log;
extern crate pokoebox_player;

use pokoebox_player::app::App;
use pokoebox_player::logger::Logger;
use pokoebox_player::manifest;

fn main() {
    // Initialize the application logger
    Logger::init().expect("Failed to initialize logger.");

    // Show an initial message
    info!("Starting {} v{}...", manifest::APP_NAME, manifest::APP_VERSION_NAME);
    info!("Developed by {}.", manifest::APP_ABOUT);

    // Create a new app instance
    let mut app = App::new().expect("Failed to initialize application.");

    // Start the application
    app.start().expect("Failed to start application.");

    // Start the main loop of the application
    app.main_loop();
}
