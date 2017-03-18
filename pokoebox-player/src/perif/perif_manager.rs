use error::Error;
use super::perif::Perif;

/// The manager for external peripherals.
pub struct PerifManager {
    perifs: Vec<Box<Perif>>
}

impl PerifManager {

    /// Construct a new peripheral manager.
    pub fn new() -> Self {
        PerifManager {
            perifs: Vec::new()
        }
    }

    /// Add the given peripheral.
    pub fn add_perif(&mut self, perif: Box<Perif>) -> Result<(), Error> {
        // Set up the peripheral, if it hasn't been set up yet
        if !perif.is_setup() {
            // Set up the peripheral
            let result = perif.setup();

            // Return errors
            if result.is_err() {
                return result;
            }
        }

        // Add the peripheral to the list
        self.perifs.push(perif);

        Ok(())
    }
}