use std::fmt::{self, Display, Formatter};
use std::sync::Arc;

use crate::app::Core;
use crate::result::Result;

/// Action trait.
///
/// Defines an action that can be invoked.
pub trait Action: Send + Sync {
    /// Get the action ID.
    fn id(&self) -> ActionId;

    /// Get the name of the action.
    /// This is a short string that should make clear to the user what the
    /// action does.
    fn name(&self) -> &'static str;

    /// Invoke the action.
    ///
    /// A boolean is returned on success which defines whether the action has
    /// been consumed. `true` if the action is consumed, `false` if not.
    /// An error is returned if the action fails.
    fn invoke(&self, core: Arc<Core>) -> Result<bool>;
}

/// Action ID.
// TODO: move somewhere else, not a trait
#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct ActionId {
    id: &'static str,
}

impl ActionId {
    /// Construct a new action ID instance.
    pub fn new(id: &'static str) -> Self {
        ActionId { id }
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
