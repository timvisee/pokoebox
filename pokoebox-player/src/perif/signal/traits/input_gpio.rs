#![cfg(feature = "rpi")]

use super::input::Input;
use super::io_gpio::IoGpio;

/// An input for a peripheral that uses GPIO features.
pub trait InputGpio: Input + IoGpio {}
