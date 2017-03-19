#![cfg(feature = "rpi")]

use super::cupi::Logic;

/// GPIO pin logic.
#[derive(Clone)]
pub enum GpioPinLogic {
    High,
    Low
}

impl GpioPinLogic {

    /// Get the logic from the given boolean value.
    pub fn from_bool(logic: bool) -> Self {
        if logic {
            GpioPinLogic::High
        } else {
            GpioPinLogic::Low
        }
    }

    /// Get the logic from the given CuPi logic.
    pub fn from_cupi(logic: Logic) -> Self {
        match logic {
            Logic::High => GpioPinLogic::High,
            Logic::Low => GpioPinLogic::Low
        }
    }

    /// Get the boolean representation for this logic value.
    pub fn as_bool(&self) -> bool {
        match *self {
            GpioPinLogic::High => true,
            GpioPinLogic::Low => false
        }
    }

    /// Convert to a boolean representation for this logic value.
    pub fn into_bool(self) -> bool {
        self.as_bool()
    }

    /// Get the CuPi logic value.
    pub fn as_cupi(&self) -> Logic {
        match *self {
            GpioPinLogic::High => Logic::High,
            GpioPinLogic::Low => Logic::Low
        }
    }

    /// Convert to a CuPi logic value.
    pub fn into_cupi(self) -> Logic {
        self.as_cupi()
    }

    /// Get the inverted logical value.
    pub fn as_inverted(&self) -> Self {
        match *self {
            GpioPinLogic::High => GpioPinLogic::Low,
            GpioPinLogic::Low => GpioPinLogic::High,
        }
    }

    /// Convert to an inverted logical value.
    pub fn invert(self) -> Self {
        self.as_inverted()
    }
}