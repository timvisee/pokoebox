use result::Result;
use super::perif_type::PerifType;

/// The manager for external peripherals.
pub struct PerifManager {
    /// List of peripherals which are managed.
    perifs: Vec<PerifType>
}

impl PerifManager {

    /// Construct a new peripheral manager.
    pub fn new() -> Self {
        PerifManager {
            perifs: Vec::new()
        }
    }

    /// Add the given peripheral.
    pub fn add_perif(&mut self, perif: PerifType) -> Result<()> {
        // Add the peripheral to the list
        self.perifs.push(perif);

        Ok(())
    }

    /// Get a list of peripherals which as being managed.
    pub fn perifs(&self) -> &Vec<PerifType> {
        &self.perifs
    }

    /// Get a list of peripherals which as being managed, mutable.
    pub fn perifs_mut(&mut self) -> &mut Vec<PerifType> {
        &mut self.perifs
    }
}
