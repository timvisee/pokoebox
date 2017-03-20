#![cfg(feature = "rpi")]

use super::sig_in_gpio::SigInGpio;
use super::sig_in_toggle::SigInToggle;

/// Signal input for a GPIO toggle.
pub trait SigInGpioToggle: SigInGpio + SigInToggle {}