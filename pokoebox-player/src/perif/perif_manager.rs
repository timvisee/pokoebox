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
        // Add the peripheral to the list
        self.perifs.push(perif);

        Ok(())
    }

    /// Get a list of peripherals which as being managed.
    pub fn perifs(&self) -> &Vec<Box<Perif>> {
        &self.perifs
    }
}