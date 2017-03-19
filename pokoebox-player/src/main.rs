#[cfg(feature = "rpi")]
extern crate cupi;
extern crate gtk;
extern crate pokoebox_player;

use pokoebox_player::gui::gui::Gui;

use gtk::prelude::*;

#[cfg(feature = "rpi")]
use cupi::{CuPi, delay_ms, DigitalWrite};
#[cfg(feature = "rpi")]
use cupi::board;

fn main() {
    #[cfg(feature = "rpi")]
    {
        use self::pokoebox_player::perif::gpio_pin::GpioPin;
        use self::pokoebox_player::perif::gpio_pin_config::{GpioPinConfig, PullMode, IoMode};
        use self::pokoebox_player::perif::gpio_pin_logic::GpioPinLogic;

        // Print the board we're using
        println!("Board: {:?}", board());

        // Set up CuPi
        let cupi = CuPi::new().unwrap();

        // Create a pin configuration
        let pin_config = GpioPinConfig::new_with_pin_and_io(0, IoMode::Output);
        let mut pinout = GpioPin::from(&cupi, pin_config).unwrap();

        loop {
            println!("Pin 0: ON");
            pinout.high();
            delay_ms(200);

            println!("Pin 0: OFF");
            pinout.low();
            delay_ms(200);
        }
    }

    // Set up the gui
    let mut gui = Gui::new().unwrap();
    gui.start();

    // Show the gui
    gui.show_master_frame();

    // Create the main grid
    let main_grid = gtk::Grid::new();
//    window.add(&main_grid);

    // Create a header
    let header = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    main_grid.attach(&header, 0, 0, 1, 1);

    // Create a button box for in the header
    let button_box = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
    header.pack_start(&button_box, false, false, 0);

    // Add a few buttons
    let button = gtk::Button::new_with_label("A");
    let button2 = gtk::Button::new_with_label("B");
    button_box.add(&button);
    button_box.add(&button2);

    // Add a close button to the end of the header
    let close_button = gtk::Button::new_with_label("X");
    header.pack_end(&close_button, false, false, 0);

    // Create the main container
    let main_container = gtk::Viewport::new(None, None);
    main_container.set_border_width(10);
    main_container.set_hexpand(true);
    main_container.set_vexpand(true);
    main_grid.attach(&main_container, 0, 1, 1, 1);

    // Create a center button
    let button3 = gtk::Button::new_with_label("Button 3!");
    main_container.add(&button3);

    // Show the application and run GTKs main loop
//    window.show_all();
    gtk::main();
}
