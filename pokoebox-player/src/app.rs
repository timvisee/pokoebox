use std::sync::Arc;

use crate::action::ActionRuntime;
use crate::result::Result;
use crate::ui::gtk::Ui;

use super::pages::PageController;

pub struct App {
    ui: Ui,
    pub core: Arc<Core>,
}

impl App {
    pub fn new() -> Result<Self> {
        // Init app core
        let core = Arc::new(Core::new()?);

        Ok(Self {
            ui: Ui::new(core.clone())?,
            core,
        })
    }

    pub fn run(self) -> Self {
        // Run the GUIs main loop
        loop {
            self.ui.main();
        }
    }
}

pub struct Core {
    /// Action manager
    pub actions: ActionRuntime,

    pub pages: PageController,
}

impl Core {
    pub fn new() -> Result<Self> {
        Ok(Self {
            actions: ActionRuntime::default(),
            pages: PageController::new(),
        })
    }
}
