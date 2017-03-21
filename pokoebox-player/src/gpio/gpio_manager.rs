#![cfg(feature = "rpi")]

use super::cupi::CuPi;

use error::Error;

/// GPIO manager.
pub struct GpioManager {
    cupi: CuPi
}

impl GpioManager {
    /// Constructor.
    pub fn new() -> Result<Self, Error> {
        // Initialize CuPi
        let cupi = CuPi::new();
        if cupi.is_err() {
            return Err(Error::new("Failed to initialize CuPi for GPIO."));
        }

        // Construct and return
        GpioManager {
            cupi: cupi
        }
    }

    /// Get the CuPi instance.
    pub fn cupi(&self) -> &CuPi {
        &self.cupi
    }
}