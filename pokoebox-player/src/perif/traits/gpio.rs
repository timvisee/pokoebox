#![cfg(feature = "old-rpi")]

use super::perif::Perif;

/// A peripheral that uses GPIO functionality.
pub trait Gpio: Perif {}
