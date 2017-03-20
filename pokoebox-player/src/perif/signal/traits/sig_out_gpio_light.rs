#![cfg(feature = "rpi")]

use super::sig_out_gpio::SigOutGpio;
use super::sig_out_light::SigOutLight;

/// A peripheral output for a light that is using GPIO.
pub trait SigOutGpioLight: SigOutGpio + SigOutLight {}
