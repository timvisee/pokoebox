#![cfg(feature = "rpi")]

use result::Result;
use super::trigger_edge::TriggerEdge;

/// Event handler trait, to handle GPIO events.
pub trait EventHandler {
    /// Invoke the event.
    /// Returns whether the event was consumed or not.
    /// `true` is returned if the event is consumed/used, `false` if it isn't.
    /// When `false` is returned, another handler is chosen to handle the event.
    fn invoke(&self) -> Result<bool>;

    /// Get the trigger edge type for this event.
    /// This defines on what signalling edges this event should be fired.
    fn trigger_edge(&self) -> TriggerEdge;
}