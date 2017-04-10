#![cfg(feature = "rpi")]

use std::fmt::{Display, Formatter, Result};

use super::logic::Logic;

/// Possible edges to trigger at.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum TriggerEdge {
    /// Trigger when the edge rises.
    Rising,

    /// Trigger when the edge falls.
    Falling,

    /// Trigger when the edge rises or falls.
    Both,
}

impl TriggerEdge {
    /// Get the trigger edge type based on a signal change.
    /// If the signal didn't change, `None` is returned.
    ///
    /// The following is returned for these signal pairs:
    ///
    /// * `Logic::Low` to `Logic::High` returns `Some(TriggerEdge::Rising)`
    /// * `Logic::High` to `Logic::Low` returns `Some(TriggerEdge::Falling)`
    pub fn from_signal_change(from: Logic, to: Logic) -> Option<TriggerEdge> {
        // Return nothing if the signal didn't change
        if from == to {
            return None;
        }

        // Return the proper trigger edge
        Some(if from.as_bool() {
            TriggerEdge::Falling
        } else {
            TriggerEdge::Rising
        })
    }

    /// Get the name of this trigger edge type.
    pub fn name(&self) -> &'static str {
        match *self {
            TriggerEdge::Rising => "Rising",
            TriggerEdge::Falling => "Falling",
            TriggerEdge::Both => "Both",
        }
    }

    /// Check whether the current edge covers the given `other` edge.
    ///
    /// This follows these rules:
    ///
    /// * `Rising` -> `true` if `other` == `Rising`
    /// * `Falling` -> `true` if `other` == `Falling`
    /// * `Both` -> `true`
    pub fn covers(&self, other: TriggerEdge) -> bool {
        // Always when the other is the same
        if *self == other {
            return true;
        }

        // True if the current edge is for both
        match *self {
            TriggerEdge::Both => true,
            _ => false
        }
    }

    /// Concatenate this trigger edge with another trigger edge.
    /// This method can be used to determine what edge type to globally listen for, when multiple
    /// events are registered with different trigger edge types.
    ///
    /// The following rules are applied:
    ///
    /// * If `self` is equal to `other`, the type of `self` is returned.
    /// * In any other case, `TriggerEdge::Both` is returned.
    ///
    /// This automatically results in the following semantics:
    ///
    /// * Concatenating the same types doesn't change the type.
    /// * When any type is `TriggerEdge::Both`, `TriggerEdge::Both` is returned.
    pub fn concat(&self, other: TriggerEdge) -> TriggerEdge {
        // Always when the other is the same
        if *self == other {
            return *self;
        }

        // Return both in all other cases
        return TriggerEdge::Both;
    }
}

/// Make a pin token displayable.
impl Display for TriggerEdge {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
