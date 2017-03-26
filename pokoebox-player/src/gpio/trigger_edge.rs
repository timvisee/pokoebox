/// Possible edges to trigger at.
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum TriggerEdge {
    /// Trigger when the edge rises.
    Rising,

    /// Trigger when the edge falls.
    Falling,

    /// Trigger when the edge rises or falls.
    Both,
}

impl TriggerEdge {
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
}