#![cfg(feature = "rpi")]

use gpio::pin_accessor::PinAccessor;
use result::Result;
use super::sig_in_gpio::SigInGpio;
use super::sig_in_toggle::SigInToggle;

/// Signal input for a GPIO toggle.
pub trait SigInGpioToggle: SigInGpio + SigInToggle {
    /// Get the state of the toggle as a boolean.
    /// When using this for a button,
    /// true might be returned when the button is pressed.
    fn state(&self, pin_accessor: &PinAccessor) -> Result<bool>;
}