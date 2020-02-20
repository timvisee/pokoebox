use std::sync::Arc;

use crate::action::prelude::*;
use crate::app::Core;
use crate::pages::PageType;
use crate::result::Result;

/// Unique ID of this action.
pub const ACTION_ID: &str = "goto-page-action";

/// Name of this action.
pub const ACTION_NAME: &str = "Goto page action";

/// Go to page action.
pub struct GotoPageAction(PageType);

impl GotoPageAction {
    pub fn new(page: PageType) -> Self {
        Self(page)
    }

    pub fn new_home() -> Self {
        Self::new(PageType::Launchpad)
    }
}

impl Action for GotoPageAction {
    fn id(&self) -> ActionId {
        ActionId::new(ACTION_ID)
    }

    fn name(&self) -> &'static str {
        ACTION_NAME
    }

    fn invoke(&self, core: Arc<Core>) -> Result<bool> {
        // TODO: propagate errors here!
        core.pages.goto_page(self.0).expect("failed to goto page");
        Ok(true)
    }
}
