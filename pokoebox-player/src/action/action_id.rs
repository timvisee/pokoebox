use std::fmt;
use std::fmt::{Display, Formatter};

/// Action ID.
#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct ActionId {
    id: &'static str
}

impl ActionId {
    /// Construct a new action ID instance.
    pub fn new(id: &'static str) -> Self {
        ActionId {
            id: id
        }
    }

    /// Get the action ID as a string.
    pub fn id(&self) -> &'static str {
        self.id
    }
}

/// Make the action ID displayable.
impl Display for ActionId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}