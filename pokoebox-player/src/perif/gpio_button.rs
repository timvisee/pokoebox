use super::gpio_perif::GpioPerif;
use super::perif::Perif;

/// Trait for a GPIO connected button.
pub struct GpioButton {
    name: &'static str
}

impl GpioButton {

    /// Construct a new GPIO button.
    // TODO: Also supply the GPIO pin that is used.
    pub fn new(name: &'static str) -> Self {
        GpioButton {
            name: name
        }
    }
}

impl GpioPerif for GpioButton {}

impl Perif for GpioButton {

    fn name(&self) -> &'static str {
        &self.name
    }

    fn setup(&self) -> Result<(), Error> {
        // TODO: Set up the button pins here!
        unimplemented!()
    }

    fn is_setup(&self) -> bool {
        // TODO: Check whether the button has been set up
        unimplemented!()
    }
}

