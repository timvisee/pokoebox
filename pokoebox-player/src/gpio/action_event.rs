#![cfg(feature = "old-rpi")]

use std::sync::{Arc, Mutex};

use super::event_handler::EventHandler;
use super::trigger_edge::TriggerEdge;
use action::action_id::ActionId;
use action::action_manager::ActionManager;
use result::Result;

/// Action event.
/// This event is used to invoke an action when the event is triggered.
pub struct ActionEvent {
    /// The ID of the action to invoke.
    action_id: ActionId,

    /// Action manager reference.
    action_manager: Arc<Mutex<ActionManager>>,

    /// The signal edge to trigger the event at.
    trigger_edge: TriggerEdge,
}

impl ActionEvent {
    /// Constructor.
    /// The ID of the action to invoke should be passed to the `action_id` parameter.
    pub fn new(
        action_id: ActionId,
        action_manager: Arc<Mutex<ActionManager>>,
        trigger_edge: TriggerEdge,
    ) -> Self {
        ActionEvent {
            action_id,
            action_manager,
            trigger_edge,
        }
    }
}

impl EventHandler for ActionEvent {
    fn invoke(&self) -> Result<bool> {
        // Get a guard on the action manager
        let guard = self.action_manager.lock().unwrap();

        // Invoke the action
        guard.invoke_action(self.action_id)
    }

    fn invoke_mut(&mut self) -> Result<bool> {
        self.invoke()
    }

    fn trigger_edge(&self) -> TriggerEdge {
        self.trigger_edge
    }
}
