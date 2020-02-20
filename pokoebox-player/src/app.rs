use std::sync::Arc;

use crate::action::ActionManager;
use crate::result::Result;
use crate::ui::gtk::Ui;

pub struct App {
    ui: Ui,
    core: Arc<Core>,
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
    pub actions: ActionManager,
}

impl Core {
    pub fn new() -> Result<Self> {
        Ok(Self {
            actions: ActionManager::new(),
        })
    }
}
