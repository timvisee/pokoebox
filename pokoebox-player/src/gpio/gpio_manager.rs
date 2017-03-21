#![cfg(feature = "rpi")]

/// GPIO manager.
pub struct GpioManager;

impl GpioManager {
    /// Constructor.
    pub fn new() -> Self {
        GpioManager {}
    }
}