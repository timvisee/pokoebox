use std::collections::HashMap;

use super::action::Action;
use super::action_id::ActionId;

/// A struct to manage all available actions.
pub struct ActionManager {
    /// List of available actions.
    actions: HashMap<ActionId, Box<Action>>,
}

impl ActionManager {
    /// Create a new action manager.
    pub fn new() -> Self {
        ActionManager {
            actions: HashMap::new()
        }
    }

    /// Add the given action to the manager.
    pub fn add_action(&mut self, action: Box<Action>) {
        self.actions.insert(action.as_ref().id(), action);
    }

    /// Find a boxed action by it's ID.
    pub fn action(&self, id: ActionId) -> Option<&Box<Action>> {
        self.actions.get(&id)
    }

    /// Find an action reference by it's ID.
    pub fn action_ref(&self, id: ActionId) -> Option<&Action> {
        if let Some(action) = self.action(id) {
            Some(action.as_ref())
        } else {
            None
        }
    }
}
