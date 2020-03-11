use std::sync::Arc;

use crate::action::prelude::*;
use crate::app::Core;
use crate::message::Message;

pub struct ActionRuntime;

impl ActionRuntime {
    // TODO: update docs
    /// Invoke the action with the given ID.
    /// A boolean is returned on success which defines whether the action has
    /// been consumed. `true` if the action has been consumed, `false` if not.
    /// If no action is available with the given ID, `false` is returned.
    /// An error is returned if the actions fails.
    pub fn invoke<A>(&self, action: A, core: Arc<Core>) -> bool
    where
        A: Action,
    {
        // Show dialog on error
        match action.invoke(core.clone()) {
            Ok(consumed) => consumed,
            Err(err) => {
                core.show_message(Message::Error(format!(
                    "Failed to invoke '{}' action.\n\nError: {}",
                    action.name(),
                    err
                )));
                false
            }
        }
    }
}

impl Default for ActionRuntime {
    fn default() -> Self {
        Self {}
    }
}
