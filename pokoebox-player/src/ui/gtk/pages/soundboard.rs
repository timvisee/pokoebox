use std::sync::Arc;

use gtk::prelude::*;

use crate::app::Core;
use crate::pages::PageType;

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

    fn build_page(&self, _core: Arc<Core>) {
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
        btn_kick.connect_clicked(|_| play(Sound::Kick));
        btns.attach(&btn_kick, 0, 0, 1, 1);

        let btn_guitar = gtk::Button::new_with_label("Guitar");
        btn_guitar.connect_clicked(|_| play(Sound::Guitar));
        btns.attach(&btn_guitar, 1, 0, 1, 1);

        let btn_c = gtk::Button::new_with_label("");
        btn_c.set_sensitive(false);
        btns.attach(&btn_c, 2, 0, 1, 1);

        let btn_d = gtk::Button::new_with_label("");
        btn_d.set_sensitive(false);
        btns.attach(&btn_d, 0, 1, 1, 1);

        let btn_e = gtk::Button::new_with_label("");
        btn_e.set_sensitive(false);
        btns.attach(&btn_e, 1, 1, 1, 1);

        let btn_f = gtk::Button::new_with_label("");
        btn_f.set_sensitive(false);
        btns.attach(&btn_f, 2, 1, 1, 1);
    }

    fn gtk_widget(&self) -> &gtk::Grid {
        &self.container
    }
}

use rodio::Source;
use std::io::Cursor;

/// Available sound types.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Sound {
    Kick,
    Guitar,
}

fn play(sound: Sound) {
    // Select sound
    let sound: &[u8] = match sound {
        Sound::Kick => include_bytes!("../../../../../res/sounds/kick_30hz.ogg"),
        Sound::Guitar => include_bytes!("../../../../../res/sounds/guitar.ogg"),
    };

    // Select output device, create source, play audio
    let device = rodio::default_output_device().unwrap();
    let source = rodio::Decoder::new(Cursor::new(sound)).unwrap();
    rodio::play_raw(&device, source.convert_samples());
}
