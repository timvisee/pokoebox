#![cfg(feature = "rpi")]

use gpio::gpio_manager::PinAccessor;
use result::Result;
use super::button::Button;
use super::gpio::Gpio;

pub trait GpioButton: Gpio + Button {
    /// Check whether the button is pressed.
    fn is_pressed(&self, pin_accessor: &PinAccessor) -> Result<bool>;
}
