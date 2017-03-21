use super::{Action, ActionId};

/// Unique ID of this action.
pub const ACTION_ID: &'static str = "test-action";

/// Name of this action.
pub const ACTION_NAME: &'static str = "Test action";

/// Test action.
pub struct TestAction;

impl TestAction {}

impl Action for TestAction {
    fn id(&self) -> ActionId {
        ACTION_ID
    }

    fn name(&self) -> &'static str {
        ACTION_NAME
    }
}