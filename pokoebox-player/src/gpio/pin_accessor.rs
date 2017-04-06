#![cfg(feature = "rpi")]

use std::collections::hash_map::{Values, ValuesMut};
use std::sync::{Mutex, MutexGuard};
use std::collections::HashMap;

use super::pin::Pin;
use super::pin_token::PinToken;

/// An accessor used to access pin data.
/// This accessor holds an exclusive lock on the available pins until it's dropped,
/// to prevent forms of data races.
pub struct PinAccessor<'a> {
    /// The mutex guard.
    guard: MutexGuard<'a, HashMap<PinToken, Pin>>,
}

impl<'a> PinAccessor<'a> {
    /// Construct a new accessor.
    pub fn new(guard: MutexGuard<'a, HashMap<PinToken, Pin>>) -> Self {
        PinAccessor {
            guard,
        }
    }

    /// Create a pin accessor instance, that provides accessibility to the pins in a safe way.
    /// The accessor is created for the given list of `pins`.
    ///
    /// This method creates a lock on the list of managed pins, to ensure concurrency safety.
    /// The lock is automatically released when the pin accessor is dropped.
    ///
    /// If an existing lock is active, the method blocks until a lock can be successfully acquired.
    pub fn from(pins: &'a Mutex<HashMap<PinToken, Pin>>) -> PinAccessor<'a> {
        PinAccessor::new(pins.lock().unwrap())
    }

    /// Add the given pin to the manager.
    /// A reference to the added pin is returned.
    pub fn add_pin(&mut self, pin: Pin) -> &Pin {
        // Store the pin token
        let token = pin.token();

        // Insert the pin, and return a reference
        self.guard.insert(token, pin);
        self.pin(token).unwrap()
    }

    /// Get an iterator over the available pins.
    pub fn pins(&self) -> Values<PinToken, Pin> {
        self.guard.values()
    }

    /// Get a mutable iterator over the available pins.
    pub fn pins_mut(&mut self) -> ValuesMut<PinToken, Pin> {
        self.guard.values_mut()
    }

    /// Get a registered pin by it's pin token.
    /// `None` is returned if the pin couldn't be found.
    pub fn pin(&self, token: PinToken) -> Option<&Pin> {
        self.guard.get(&token)
    }

    /// Get a mutable registered pin by it's pin token.
    /// `None` is returned if the pin couldn't be found.
    pub fn pin_mut(&mut self, token: PinToken) -> Option<&mut Pin> {
        self.guard.get_mut(&token)
    }
}
