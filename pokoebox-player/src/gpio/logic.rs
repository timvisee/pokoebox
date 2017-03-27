#![cfg(feature = "rpi")]

use super::cupi;

/// GPIO pin logic.
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Logic {
    High,
    Low
}

impl Logic {

    /// Get the logic from the given boolean value.
    pub fn from_bool(logic: bool) -> Self {
        if logic {
            Logic::High
        } else {
            Logic::Low
        }
    }

    /// Get the logic from the given CuPi logic.
    pub fn from_cupi(logic: cupi::Logic) -> Self {
        match logic {
            cupi::Logic::High => Logic::High,
            cupi::Logic::Low => Logic::Low
        }
    }

    /// Get the boolean representation for this logic value.
    pub fn as_bool(&self) -> bool {
        match *self {
            Logic::High => true,
            Logic::Low => false
        }
    }

    /// Convert to a boolean representation for this logic value.
    pub fn into_bool(self) -> bool {
        self.as_bool()
    }

    /// Get the CuPi logic value.
    pub fn as_cupi(&self) -> cupi::Logic {
        match *self {
            Logic::High => cupi::Logic::High,
            Logic::Low => cupi::Logic::Low
        }
    }

    /// Convert to a CuPi logic value.
    pub fn into_cupi(self) -> cupi::Logic {
        self.as_cupi()
    }

    /// Get the inverted logical value.
    pub fn as_inverted(&self) -> Self {
        match *self {
            Logic::High => Logic::Low,
            Logic::Low => Logic::High,
        }
    }

    /// Convert to an inverted logical value.
    pub fn into_inverted(self) -> Self {
        self.as_inverted()
    }
}

unsafe impl Send for Logic {}

unsafe impl Sync for Logic {}