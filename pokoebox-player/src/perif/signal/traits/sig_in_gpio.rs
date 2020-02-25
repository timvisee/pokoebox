#![cfg(feature = "old-rpi")]

use super::sig_gpio::SigGpio;
use super::sig_in::SigIn;

/// An input for a peripheral that uses GPIO features.
pub trait SigInGpio: SigIn + SigGpio {}
