#![cfg(feature = "rpi")]

use action::action_id::ActionId;
use super::event_handler::EventHandler;
use super::trigger_edge::TriggerEdge;

/// Action event.
/// This event is used to invoke an action when the event is triggered.
pub struct ActionEvent {
    /// The ID of the action to invoke.
    action_id: ActionId,

    /// The signal edge to trigger the event at.
    trigger_edge: TriggerEdge,
}

impl ActionEvent {
    /// Constructor.
    /// The ID of the action to invoke should be passed to the `action_id` parameter.
    pub fn new(action_id: ActionId) -> Self {
        ActionEvent {
            action_id,
            trigger_edge,
        }
    }
}

impl EventHandler for ActionEvent {
    fn invoke(&self) -> Result<bool> {
        // TODO: Invoke the action here!
        // TODO: Should these actions only be invoked mutably?
        unimplemented!()
    }

    fn invoke_mut(&mut self) -> Result<bool> {
        // TODO: Invoke the action here!
        unimplemented!()
    }

    fn trigger_edge(&self) -> TriggerEdge {
        self.trigger_edge
    }
}