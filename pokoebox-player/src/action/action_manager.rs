use std::collections::HashMap;

use super::action_id::ActionId;
use super::actions::test_action::TestAction;
use super::Action;
use result::Result;

/// A struct to manage all available actions.
pub struct ActionManager {
    /// List of available actions.
    actions: HashMap<ActionId, Box<dyn Action>>,
}

impl ActionManager {
    /// Create a new action manager.
    pub fn new() -> Self {
        ActionManager {
            actions: HashMap::new(),
        }
    }

    /// Load a fixed list of 'normal' actions.
    /// Some actions require extra metadata and/or parameters,
    /// those must be loaded manually.
    pub fn load_normal_actions(&mut self) {
        info!("Loading normal actions...");

        self.add_action(Box::new(TestAction::new()));

        info!("{} actions loaded.", self.actions.len());
    }

    /// Add the given action to the manager.
    pub fn add_action(&mut self, action: Box<dyn Action>) {
        debug!("Adding action with ID '{}'...", action.as_ref().id());
        self.actions.insert(action.as_ref().id(), action);
    }

    /// Find a boxed action by it's ID.
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
    pub fn invoke_action(&self, id: ActionId) -> Result<bool> {
        if let Some(action) = self.action_ref(id) {
            action.invoke()
        } else {
            Ok(false)
        }
    }
}
