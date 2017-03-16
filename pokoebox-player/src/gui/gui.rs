extern crate gtk;

use error::Error;

/// Main gui object, which manages the graphical interface side of the application.
///
/// Creating a new instance will automatically initialize GTK.
pub struct Gui { }

impl Gui {

    /// Constructor.
    ///
    /// Constructing the object will initialize the GTK toolkit.
    ///
    /// # Errors
    ///
    /// Returns an error if GTK failed to initialize, blocking further GTK usage.
    pub fn new() -> Result<Self, Error> {
        // Initialize GTK, handle errors
        if gtk::init().is_err() || true {
            return Err(Error::new("Failed to initialize GTK"));
        }

        // Build the object and return it
        Ok(Gui { })
    }
}