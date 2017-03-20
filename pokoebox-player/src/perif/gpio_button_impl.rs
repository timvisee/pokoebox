#![cfg(feature = "rpi")]

use error::Error;
use std::collections::HashMap;
use super::cupi::PinInput;
use super::gpio_perif::GpioPerifInputs;
use super::perif::Perif;

/// Input pin key for the button pin.
pub const INPUT_PIN_KEY_BUTTON: &'static str = "button";

/// Trait for a GPIO connected button.
pub struct GpioButtonImpl {
    name: &'static str,
    inputs: HashMap<&'static str, PinInput>
}

impl GpioButtonImpl {

    /// Construct a new GPIO button.
    pub fn new(name: &'static str, input_pin: PinInput) -> Self {
        // Create the struct
        let mut gpio_button = GpioButtonImpl {
            name: name,
            inputs: HashMap::new()
        };

        // Add the input
        gpio_button.inputs.insert(INPUT_PIN_KEY_BUTTON, input_pin);

        gpio_button
    }

    /// Get the input pin for the button.
    pub fn get_button_pin(&self) -> Option<&PinInput> {
        self.inputs.get(INPUT_PIN_KEY_BUTTON)
    }
}

impl GpioPerifInputs for GpioButtonImpl {

    fn input_pins(&self) -> &HashMap<&'static str, PinInput> {
        &self.inputs
    }
}

impl Perif for GpioButtonImpl {

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

