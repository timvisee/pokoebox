#![cfg(feature = "rpi")]

use super::sig_gpio::SigGpio;
use super::sig_out::SigOut;

/// An output for a peripheral that uses GPIO features.
pub trait SigOutGpio: SigOut + SigGpio {}
