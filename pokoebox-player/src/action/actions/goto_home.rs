use crate::action::prelude::*;
use crate::result::Result;

/// Unique ID of this action.
pub const ACTION_ID: &str = "goto-home-action";

/// Name of this action.
pub const ACTION_NAME: &str = "Goto home action";

/// Go to home action.
pub struct GotoHomeAction;

impl Default for GotoHomeAction {
    fn default() -> Self {
        Self
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
