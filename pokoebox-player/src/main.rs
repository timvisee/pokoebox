extern crate gtk;

use gtk::prelude::*;

fn main() {
    // Initialize GTK and make sure it's ok
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // Create a window
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("PokoeBox Player");
    window.set_border_width(0);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(640, 480);

    // Connect the delete event
    window.connect_delete_event(|_, _| {
        // Close the application
        gtk::main_quit();
        Inhibit(false)
    });

//    let info = gtk::InfoBar::new();
//    info.add_button("Some button", 0);
//    window.add(&info);

    // Create the main grid
    let main_grid = gtk::Grid::new();
    window.add(&main_grid);

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
    window.show_all();
    gtk::main();
}
