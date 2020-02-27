use std::sync::Arc;

use gtk::prelude::*;

use crate::app::Core;
use crate::pages::PageType;
use crate::soundeffecter::Sound;

use super::page::Helper;
use super::page::Page;

const PAGE_TYPE: PageType = PageType::Soundboard;
const PAGE_NAME: &str = "Soundboard";
const BUTTON_SPACING: u32 = 16;
const BUTTON_GRID_SIZE: (i32, i32) = (450, 260);

/// Soundboard.
pub struct Soundboard {
    container: gtk::Grid,
}

impl Soundboard {
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

impl Page for Soundboard {
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
        btns.set_size_request(BUTTON_GRID_SIZE.0, BUTTON_GRID_SIZE.1);
        self.container.add(&btns);

        // Add some buttons
        let btn_kick = gtk::Button::new_with_label("Kick 30Hz");
        btn_kick.connect_clicked(play(core.clone(), Sound::Kick));
        btns.attach(&btn_kick, 0, 0, 1, 1);

        let btn_guitar = gtk::Button::new_with_label("Guitar");
        btn_guitar.connect_clicked(play(core.clone(), Sound::Guitar));
        btns.attach(&btn_guitar, 1, 0, 1, 1);

        let btn_xp = gtk::Button::new_with_label("XP");
        btn_xp.connect_clicked(play(core.clone(), Sound::Xp));
        btns.attach(&btn_xp, 2, 0, 1, 1);

        let btn_jbl = gtk::Button::new_with_label("JBL");
        btn_jbl.connect_clicked(play(core.clone(), Sound::Jbl));
        btns.attach(&btn_jbl, 0, 1, 1, 1);

        let btn_car = gtk::Button::new_with_label("Mustang");
        btn_car.connect_clicked(play(core.clone(), Sound::MustangStart));
        btns.attach(&btn_car, 1, 1, 1, 1);

        let btn_f = gtk::Button::new_with_label("");
        btn_f.set_sensitive(false);
        btns.attach(&btn_f, 2, 1, 1, 1);
    }

    fn gtk_widget(&self) -> &gtk::Grid {
        &self.container
    }
}

/// Helper to easily play sounds from buttons.
fn play(core: Arc<Core>, sound: Sound) -> impl Fn(&gtk::Button) + 'static {
    move |_| {
        if let Err(err) = core.effecter.play(sound) {
            error!("Failed to play soundboard sound: {:?}", err);
        }
    }
}
