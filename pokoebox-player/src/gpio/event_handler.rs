#![cfg(feature = "rpi")]

use result::Result;

/// Event handler trait, to handle GPIO events.
pub trait EventHandler {
    /// Handle an event with the given token.
    /// Returns whether the event was consumed or not.
    /// `true` is returned if the event is consumed/used, `false` if it isn't.
    /// When `false` is returned, another handler is chosen to handle the event.
    fn handle(&self, token: usize) -> Result<bool>;
}