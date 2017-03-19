#![cfg(feature = "rpi")]

use error::Error;
use std::collections::HashMap;
use super::cupi::PinInput;
use super::gpio_perif_inputs::GpioPerifInputs;
use super::perif::Perif;

/// Input pin key for the button pin.
pub const INPUT_PIN_KEY_BUTTON: &'static str = "button";

/// Trait for a GPIO connected button.
pub struct GpioButton {
    name: &'static str,
    inputs: HashMap<&'static str, PinInput>
}

impl GpioButton {

    /// Construct a new GPIO button.
    // TODO: Also supply the GPIO pin that is used.
    pub fn new(name: &'static str) -> Self {
        GpioButton {
            name: name,
            inputs: HashMap::new()
        }
    }

    /// Get the input pin for the button.
    pub fn get_button_pin(&self) -> Option<&PinInput> {
        self.inputs.get(INPUT_PIN_KEY_BUTTON)
    }
}

impl GpioPerifInputs for GpioButton {

    fn input_pins(&self) -> &HashMap<&'static str, PinInput> {
        &self.inputs
    }
}

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

