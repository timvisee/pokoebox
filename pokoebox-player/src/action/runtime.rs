use std::sync::Arc;

use crate::action::prelude::*;
use crate::app::Core;
use crate::result::Result;

pub struct ActionRuntime;

impl ActionRuntime {
    // TODO: update docs
    /// Invoke the action with the given ID.
    /// A boolean is returned on success which defines whether the action has
    /// been consumed. `true` if the action has been consumed, `false` if not.
    /// If no action is available with the given ID, `false` is returned.
    /// An error is returned if the actions fails.
    pub fn invoke<A>(&self, action: A, core: Arc<Core>) -> Result<bool>
    where
        A: Action,
    {
        action.invoke(core)
    }
}

impl Default for ActionRuntime {
    fn default() -> Self {
        Self {}
    }
}
