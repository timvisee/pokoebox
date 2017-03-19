#![cfg(feature = "rpi")]

use super::cupi::Logic;

/// GPIO pin logic.
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

    /// Get the boolean representation for this value.
    pub fn as_bool(&self) -> bool {
        match *self {
            GpioPinLogic::High => true,
            GpioPinLogic::Low => false
        }
    }

    /// Get the CuPi logic value.
    pub fn as_cupi(&self) -> Logic {
        match *self {
            GpioPinLogic::High => Logic::High,
            GpioPinLogic::Low => Logic::Low
        }
    }

    /// Convert to a CuPi logic value.
    pub fn to_cupi(self) -> Logic {
        self.as_cupi()
    }
}