#[macro_use]
extern crate log;

pub mod action;
pub mod app;
pub mod error;
pub mod pages;
pub mod perif;
pub mod result;
pub mod soundeffecter;
pub mod ui;
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

    // Build and run app
    App::new().expect("failed to initialize application.").run();
}
