use std::sync::Arc;

use crate::action::ActionManager;
use crate::result::Result;
use crate::ui::gtk::Ui;

pub struct App {
    ui: Ui,
    state: Arc<State>,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            ui: Ui::new()?,
            state: Arc::new(State::new()?),
        })
    }

    pub fn run(self) -> Self {
        // Run the GUIs main loop
        loop {
            self.ui.main();
        }
    }
}

pub struct State {
    /// Action manager
    _actions: ActionManager,
}

impl State {
    pub fn new() -> Result<Self> {
        Ok(Self {
            _actions: ActionManager::new(),
        })
    }
}
