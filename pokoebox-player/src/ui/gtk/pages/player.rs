use std::sync::Arc;

use gtk::prelude::*;
use pokoebox_media::mpris::Cmd;

use crate::app::Core;
use crate::pages::PageType;

use super::page::Helper;
use super::page::Page;

const PAGE_TYPE: PageType = PageType::Player;
const PAGE_NAME: &str = "Player";
const BUTTON_SPACING: u32 = 16;
const BUTTON_SIZE: (i32, i32) = (80, 80);

/// Player page.
pub struct Player {
    /// Page container
    container: gtk::Grid,
}

impl Player {
    /// Constructor.
    pub fn new(core: Arc<Core>) -> Self {
        // Create the page instance
        let page = Self {
            container: Helper::create_page_container(),
        };

        // Build the ui
        page.build_page(core);

        page
    }
}

impl Page for Player {
    fn page_type(&self) -> PageType {
        PAGE_TYPE
    }

    fn page_name(&self) -> &'static str {
        &PAGE_NAME
    }

    fn build_page(&self, core: Arc<Core>) {
        // Configure the page
        self.container.set_halign(gtk::Align::Center);
        self.container.set_valign(gtk::Align::Center);

        // Create a button grid
        let btns = gtk::Grid::new();
        btns.set_row_spacing(BUTTON_SPACING);
        btns.set_column_spacing(BUTTON_SPACING);
        btns.set_row_homogeneous(true);
        btns.set_column_homogeneous(true);
        self.container.add(&btns);

        let btn_prev = gtk::Button::new_from_icon_name(
            Some("media-skip-backward"),
            gtk::IconSize::LargeToolbar,
        );
        btn_prev.set_size_request(BUTTON_SIZE.0, BUTTON_SIZE.1);
        let closure_core = core.clone();
        btn_prev.connect_clicked(move |_| {
            // TODO: handle result
            closure_core
                .mpris
                .send_cmd(Cmd::Previous)
                .expect("failed to send signal");
        });
        btns.add(&btn_prev);

        let btn_play = gtk::Button::new_from_icon_name(
            Some("media-playback-start"),
            gtk::IconSize::LargeToolbar,
        );
        btn_play.set_size_request(BUTTON_SIZE.0, BUTTON_SIZE.1);
        let closure_core = core.clone();
        btn_play.connect_clicked(move |_| {
            // TODO: handle result
            closure_core
                .mpris
                .send_cmd(Cmd::Play)
                .expect("failed to send signal");
        });
        btns.add(&btn_play);

        let btn_stop = gtk::Button::new_from_icon_name(
            Some("media-playback-stop"),
            gtk::IconSize::LargeToolbar,
        );
        btn_stop.set_size_request(BUTTON_SIZE.0, BUTTON_SIZE.1);
        let closure_core = core.clone();
        btn_stop.connect_clicked(move |_| {
            // TODO: handle result
            closure_core
                .mpris
                .send_cmd(Cmd::Pause)
                .expect("failed to send signal");
        });
        btns.add(&btn_stop);

        let btn_fwd = gtk::Button::new_from_icon_name(
            Some("media-skip-forward"),
            gtk::IconSize::LargeToolbar,
        );
        btn_fwd.set_size_request(BUTTON_SIZE.0, BUTTON_SIZE.1);
        let closure_core = core.clone();
        btn_fwd.connect_clicked(move |_| {
            // TODO: handle result
            closure_core
                .mpris
                .send_cmd(Cmd::Next)
                .expect("failed to send signal");
        });
        btns.add(&btn_fwd);
    }

    fn gtk_widget(&self) -> &gtk::Grid {
        &self.container
    }
}
