#![cfg(feature = "rpi")]

use super::sig_in::SigIn;
use super::sig_gpio::SigGpio;

/// An input for a peripheral that uses GPIO features.
pub trait SigInGpio: SigIn + SigGpio {}
