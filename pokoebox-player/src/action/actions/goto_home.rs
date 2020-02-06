use super::{Action, ActionId};
use crate::result::Result;

/// Unique ID of this action.
pub const ACTION_ID: &'static str = "goto-home-action";

/// Name of this action.
pub const ACTION_NAME: &'static str = "Goto home action";

/// Go to home action.
pub struct GotoHomeAction;

impl GotoHomeAction {
    pub fn new() -> Self {
        GotoHomeAction
    }
}

impl Action for GotoHomeAction {
    fn id(&self) -> ActionId {
        ActionId::new(ACTION_ID)
    }

    fn name(&self) -> &'static str {
        ACTION_NAME
    }

    fn invoke(&self) -> Result<bool> {
        // TODO: goto home page

        Ok(true)
    }
}
