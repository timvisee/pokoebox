use std::sync::Arc;

use crate::action::prelude::*;
use crate::app::Core;
use crate::result::Result;

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
    fn name(&self) -> &'static str {
        ACTION_NAME
    }

    fn invoke(&self, _core: Arc<Core>) -> Result<bool> {
        debug!("NOP action got invoked");
        Ok(true)
    }
}
