use std::collections::HashMap;

use super::action::Action;
use super::action_id::ActionId;
use super::actions::test_action::TestAction;

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

    /// Load a fixed list of 'normal' actions.
    /// Some actions require extra metadata and/or parameters, those must be loaded manually.
    pub fn load_normal_actions(&mut self) {
        info!("Loading normal actions...");

        self.add_action(Box::new(TestAction::new()));

        info!("{} actions loaded.", self.actions.len());
    }

    /// Add the given action to the manager.
    pub fn add_action(&mut self, action: Box<Action>) {
        debug!("Adding action with ID '{}'...", action.as_ref().id());
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
