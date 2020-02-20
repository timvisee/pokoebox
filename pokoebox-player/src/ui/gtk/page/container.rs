use std::rc::Rc;
use std::sync::Arc;

use gtk::prelude::*;

use crate::app::Core;

use super::manager::PageManager;

/// Container holding and managing the application pages.
pub struct Container {
    container: Rc<gtk::Notebook>,
    pub manager: Rc<PageManager>,
}

impl Container {
    pub fn new(core: Arc<Core>) -> Self {
        let container = Self::build_ui();
        let manager = Rc::new(PageManager::new(container.clone()));

        // TODO: create channel here, attach to page controller
        // TODO: use correct priority here
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let chan_manager = manager.clone();
        let chan_core = core.clone();
        rx.attach(None, move |page| {
            chan_manager.goto_page(chan_core.clone(), page);
            glib::Continue(true)
        });
        core.pages.set_channel(tx);

        Self { container, manager }
    }

    /// Build the UI container.
    fn build_ui() -> Rc<gtk::Notebook> {
        // Create the container instance
        let container = Rc::new(gtk::Notebook::new());

        // Configure the container
        container.set_hexpand(true);
        container.set_vexpand(true);
        container.set_halign(gtk::Align::Fill);
        container.set_valign(gtk::Align::Fill);
        container.set_tab_pos(gtk::PositionType::Bottom);
        container.set_show_border(false);
        container.set_scrollable(true);

        container
    }

    /// Get the GTK widget for this page container.
    pub fn gtk_widget(&self) -> &gtk::Notebook {
        &self.container
    }
}
