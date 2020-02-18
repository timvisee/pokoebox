use std::collections::HashMap;

use crate::action::{actions::*, prelude::*};
use crate::result::Result;

/// A struct to manage all available actions.
pub struct ActionManager {
    /// List of available actions.
    actions: HashMap<ActionId, Box<dyn Action>>,
}

impl ActionManager {
    pub fn new() -> Self {
        let mut manager = Self {
            actions: HashMap::default(),
        };

        // Add default list of actions
        debug!("Loading default actions...");
        manager.add(Box::new(NopAction::default()));
        manager.add(Box::new(GotoHomeAction::default()));
        debug!("{} actions loaded.", manager.actions.len());

        manager
    }

    /// Add the given action to the manager.
    pub fn add(&mut self, action: Box<dyn Action>) {
        debug!("Adding action (ID: {})'...", action.id());
        self.actions.insert(action.id(), action);
    }

    /// Find a boxed action by it's ID.
    #[allow(clippy::borrowed_box)]
    pub fn action(&self, id: ActionId) -> Option<&Box<dyn Action>> {
        self.actions.get(&id)
    }

    /// Find an action reference by it's ID.
    pub fn action_ref(&self, id: ActionId) -> Option<&dyn Action> {
        if let Some(action) = self.action(id) {
            Some(action.as_ref())
        } else {
            None
        }
    }

    /// Invoke the action with the given ID.
    /// A boolean is returned on success which defines whether the action has
    /// been consumed. `true` if the action has been consumed, `false` if not.
    /// If no action is available with the given ID, `false` is returned.
    /// An error is returned if the actions fails.
    pub fn invoke(&self, id: ActionId) -> Result<bool> {
        if let Some(action) = self.action_ref(id) {
            action.invoke()
        } else {
            Ok(false)
        }
    }
}
