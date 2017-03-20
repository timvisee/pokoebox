#![cfg(feature = "rpi")]

use super::output_gpio::OutputGpio;
use super::output_light::OutputLight;

/// A peripheral output for a light that is using GPIO.
pub trait OutputGpioLight: OutputGpio + OutputLight {}
