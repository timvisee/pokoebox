#![cfg(feature = "old-rpi")]

use super::button::Button;
use super::gpio::Gpio;
use gpio::pin_accessor::PinAccessor;
use result::Result;

pub trait GpioButton: Gpio + Button {
    /// Check whether the button is pressed.
    fn is_pressed(&self, pin_accessor: &PinAccessor) -> Result<bool>;
}
