#![cfg(feature = "rpi")]

use std::collections::HashMap;

use super::cupi::PinInput;

use super::perif::Perif;

/// GPIO peripheral trait for inputs.
pub trait GpioPerifInputs: Perif {

    /// Get a hash map of all input pins.
    fn input_pins(&self) -> &HashMap<&'static str, PinInput>;

    /// Get a single input pin.
    /// This should only be used for convenience if the peripheral has a single input pin.
    /// If the peripheral has multiple input pins, an arbitrary one is returned.
    fn to_input_pin(&self) -> Option<&PinInput> {
        self.input_pins().values().next()
    }

    /// Get a hash map of all input pins.
    fn to_input_pins_vec(&self) -> Vec<&PinInput> {
        // Create a vector to put the pins in
        let mut vec = Vec::new();

        // Add each pin
        for pin in self.input_pins().values() {
            vec.push(pin);
        }

        vec
    }
}
