extern crate gtk;

/// Main gui manager object, which manages the graphical interface side of the project.
///
/// Creating a new instance will automatically initialize GTK.
pub struct Gui { }

impl Gui {

    /// Constructor.
    ///
    ///
    pub fn new() -> Result<Self, ()> {
        // Initialize GTK and make sure it's ok
        if gtk::init().is_err() {
            // TODO: Return a proper error here.
            println!("Failed to initialize GTK.");
            return Err(());
        }

        // Build the object and return it
        Ok(Gui { })
    }
}