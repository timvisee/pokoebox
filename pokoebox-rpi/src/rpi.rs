use std::sync::{Arc, Mutex};

use rppal::i2c::I2c;

/// Raspberry Pi base.
///
/// Used to share various resources such as the i2c bus.
pub struct Rpi {
    /// Shared i2c interface.
    i2c: Option<Arc<Mutex<I2c>>>,
}

impl Rpi {
    /// Get a shared i2c bus instance.
    // TODO: propagate errors?
    pub fn get_i2c(&mut self) -> Arc<Mutex<I2c>> {
        // Get existing bus
        if let Some(ref bus) = self.i2c {
            return bus.clone();
        }

        // Obtain bus
        let bus = Arc::new(Mutex::new(
            I2c::new().expect("Failed to obtain i2c bus access"),
        ));
        self.i2c.replace(bus.clone());

        bus
    }
}

impl Default for Rpi {
    fn default() -> Self {
        Self { i2c: None }
    }
}
