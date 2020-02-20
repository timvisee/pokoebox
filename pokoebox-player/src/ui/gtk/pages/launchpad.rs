use std::sync::Arc;

use gtk::{self, prelude::*};

use crate::action::{actions::GotoHomeAction, prelude::*};
use crate::app::Core;

use super::page::Helper;
use super::page::Page;

/// Name of the page.
const PAGE_NAME: &str = "Launchpad";
const BUTTON_SPACING: u32 = 16;
const BUTTON_GRID_SIZE: (i32, i32) = (450, 260);

/// Launchpad.
pub struct Launchpad {
    container: gtk::Grid,
}

impl Launchpad {
    /// Constructor.
    pub fn new(core: Arc<Core>) -> Self {
        // Create the page instance
        let page = Self {
            container: Helper::create_page_container(),
        };

        // Build the page ui
        page.build_page(core);

        page
    }
}

impl Page for Launchpad {
    fn page_name(&self) -> &'static str {
        &PAGE_NAME
    }

    fn build_page(&self, core: Arc<Core>) {
        // Configure the page
        self.container.set_halign(gtk::Align::Center);
        self.container.set_valign(gtk::Align::Center);

        // Create a button grid
        let buttons = gtk::Grid::new();
        buttons.set_row_spacing(BUTTON_SPACING);
        buttons.set_column_spacing(BUTTON_SPACING);
        buttons.set_row_homogeneous(true);
        buttons.set_column_homogeneous(true);
        buttons.set_size_request(BUTTON_GRID_SIZE.0, BUTTON_GRID_SIZE.1);
        self.container.add(&buttons);

        // Add some buttons
        let button_play = gtk::Button::new_with_label("Play");
        button_play.connect_clicked(move |_| {
            // TODO: handle result
            let _ = core.actions.invoke(GotoHomeAction::default().id());
        });
        buttons.attach(&button_play, 0, 0, 1, 1);
        let button_b = gtk::Button::new_with_label("Button B");
        buttons.attach(&button_b, 1, 0, 1, 1);
        let button_c = gtk::Button::new_with_label("Button C");
        buttons.attach(&button_c, 2, 0, 1, 1);
        let button_d = gtk::Button::new_with_label("Button D");
        buttons.attach(&button_d, 0, 1, 1, 1);
        let button_e = gtk::Button::new_with_label("Button E");
        buttons.attach(&button_e, 1, 1, 1, 1);
        let button_settings = gtk::Button::new_with_label("Settings");
        buttons.attach(&button_settings, 2, 1, 1, 1);
    }

    fn gtk_widget(&self) -> &gtk::Grid {
        &self.container
    }
}
