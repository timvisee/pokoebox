use super::{Action, ActionId};
use crate::result::Result;

/// Unique ID of this action.
pub const ACTION_ID: &'static str = "nop-action";

/// Name of this action.
pub const ACTION_NAME: &'static str = "NOP action";

/// NOP action, doing exactly nothing.
pub struct NopAction;

impl NopAction {
    pub fn new() -> Self {
        NopAction
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
