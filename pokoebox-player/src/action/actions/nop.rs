use crate::action::prelude::*;
use crate::result::Result;

/// Unique ID of this action.
pub const ACTION_ID: &str = "nop-action";

/// Name of this action.
pub const ACTION_NAME: &str = "NOP action";

/// NOP action, doing exactly nothing.
pub struct NopAction;

impl Default for NopAction {
    fn default() -> Self {
        Self
    }
}

impl Action for NopAction {
    fn id(&self) -> ActionId {
        ActionId::new(ACTION_ID)
    }

    fn name(&self) -> &'static str {
        ACTION_NAME
    }

    fn invoke(&self) -> Result<bool> {
        debug!("NOP action got invoked");
        Ok(true)
    }
}
