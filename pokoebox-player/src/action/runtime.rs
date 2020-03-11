use std::sync::Arc;

use crate::action::prelude::*;
use crate::app::Core;
use crate::message::Message;
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
        let result = action.invoke(core.clone());

        // Show dialog on error
        if let Err(ref err) = result {
            core.show_message(Message::Error(format!(
                "Failed to invoke '{}' action.\n\nError: {}",
                action.name(),
                err
            )));
        }

        result
    }
}

impl Default for ActionRuntime {
    fn default() -> Self {
        Self {}
    }
}
