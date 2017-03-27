#![cfg(feature = "rpi")]

use gpio::gpio_manager::GpioManager;
use super::button::Button;
use super::gpio::Gpio;

pub trait GpioButton: Gpio + Button {
    /// Check whether the button is pressed.
    fn is_pressed(&self, gpio_manager: &GpioManager) -> Option<bool>;
}
