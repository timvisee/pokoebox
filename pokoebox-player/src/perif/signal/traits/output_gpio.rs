#![cfg(feature = "rpi")]

use super::io_gpio::IoGpio;
use super::output::Output;

/// An output for a peripheral that uses GPIO features.
pub trait OutputGpio: Output + IoGpio {}