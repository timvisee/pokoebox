extern crate sysfs_gpio;
extern crate gtk;
extern crate pokoebox_player;

use pokoebox_player::gui::gui::Gui;

use gtk::prelude::*;
use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Set up the gui
    let mut gui = Gui::new().unwrap();
    gui.start();

    // Show the gui
    gui.show_master_frame();



    let my_led = Pin::new(16); // number depends on chip, etc.
    my_led.with_exported(|| {
        loop {
            my_led.set_value(0).unwrap();
            sleep(Duration::from_millis(200));
            my_led.set_value(1).unwrap();
            sleep(Duration::from_millis(200));
        }
    }).unwrap();



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
