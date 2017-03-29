use std::sync::MutexGuard;
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
            guard: guard,
        }
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
