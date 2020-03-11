use std::sync::Arc;

use crate::action::prelude::*;
use crate::app::Core;
use crate::error::Error;
use crate::pages::PageType;
use crate::result::Result;

/// Name of this action.
pub const ACTION_NAME: &str = "Goto page";

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
    fn name(&self) -> &'static str {
        ACTION_NAME
    }

    fn invoke(&self, core: Arc<Core>) -> Result<bool> {
        core.pages.goto_page(self.0).map(|_| true).map_err(|err| {
            Error::new(format!(
                "Selecting requested page through page manager failed: {:?}",
                err,
            ))
        })
    }
}
