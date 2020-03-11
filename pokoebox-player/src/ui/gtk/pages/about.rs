use std::sync::Arc;

use gtk::prelude::*;

use crate::app::Core;
use crate::pages::PageType;

use super::page::Helper;
use super::page::Page;

const PAGE_TYPE: PageType = PageType::About;
const PAGE_NAME: &str = "About";
const LABEL_SPACING: u32 = 16;

/// About.
pub struct About {
    container: gtk::Grid,
}

impl About {
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

impl Page for About {
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

        // Create a label grid
        let grid = gtk::Grid::new();
        grid.set_row_spacing(LABEL_SPACING);
        grid.set_column_spacing(LABEL_SPACING);
        self.container.add(&grid);

        // Page header
        let header = gtk::LabelBuilder::new()
            .label("<b>PokoeBox</b>")
            .use_markup(true)
            .halign(gtk::Align::Center)
            .build();
        grid.attach(&header, 0, 0, 2, 1);

        // Description text
        let header_state = gtk::LabelBuilder::new()
            .label("Portable media center boombox project.")
            .halign(gtk::Align::Center)
            .build();
        grid.attach(&header_state, 0, 1, 2, 1);

        // Version header
        let header_voltage = gtk::LabelBuilder::new()
            .label("Version:")
            .halign(gtk::Align::Start)
            .build();
        grid.attach(&header_voltage, 0, 2, 1, 1);

        // Version label
        let label_voltage = gtk::LabelBuilder::new()
            .label("0.1.0")
            .halign(gtk::Align::Start)
            .build();
        grid.attach(&label_voltage, 1, 2, 1, 1);

        // Webpage header
        let header_web = gtk::LabelBuilder::new()
            .label("Webpage:")
            .halign(gtk::Align::Start)
            .build();
        grid.attach(&header_web, 0, 3, 1, 1);

        // Webpage label
        let label_web = gtk::LabelBuilder::new()
            .label("https://github.com/timvisee/pokoebox")
            .halign(gtk::Align::Start)
            .use_underline(true)
            .build();
        grid.attach(&label_web, 1, 3, 1, 1);

        // Specs header
        let header_specs = gtk::LabelBuilder::new()
            .label("Specs:")
            .valign(gtk::Align::Start)
            .halign(gtk::Align::Start)
            .build();
        grid.attach(&header_specs, 0, 4, 1, 1);

        // Specs label
        let label_specs = gtk::LabelBuilder::new()
            .label(
                "-   Amp: Volt+, Hi-Fi class D, 2x 50W\n\
                 -   DAC: Pi-DAC+, Hi-Fi, 192kHz 24-bit\n\
                 -   Screen: 1024x600, 7 inch, touch\n\
                 -   IO: 2x 3.5mm audio jack\n\
                 -   CPU: Quad-core A53 64-bit @ 1.4GHz\n\
                 -   RAM: 1GB LPDDR2 SDRAM\n\
                 -   Bluetooth: 4.2, LE\n\
                 -   Wi-Fi: 2.4GHz &amp; 5GHz 802.11b/g/n/ac\n\
                 -   Ethernet: 1Gbit\n\
                 -   Battery: 24V 220Wh",
            )
            .use_markup(true)
            .halign(gtk::Align::Start)
            .build();
        grid.attach(&label_specs, 1, 4, 1, 1);
    }

    fn gtk_widget(&self) -> &gtk::Grid {
        &self.container
    }
}
