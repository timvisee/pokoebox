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
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    // Connect the delete event
    window.connect_delete_event(|_, _| {
        // Close the application
        gtk::main_quit();
        Inhibit(false)
    });

    // Create a button and add it to the window
    let button = gtk::Button::new_with_label("Click me!");
    window.add(&button);

    // Show the application and run GTKs main loop
    window.show_all();
    gtk::main();
}
