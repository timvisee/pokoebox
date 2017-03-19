#![cfg(feature = "rpi")]

use std::collections::HashMap;

use super::cupi::PinOutput;

use super::perif::Perif;

/// GPIO peripheral trait for outputs.
pub trait GpioPerifOutputs: Perif {

    /// Get a hash map of all output pins.
    fn output_pins(&self) -> &HashMap<&'static str, PinOutput>;

    /// Get a single output pin.
    /// This should only be used for convenience if the peripheral has a single output pin.
    /// If the peripheral has multiple output pins, an arbitrary one is returned.
    fn to_output_pin(&self) -> Option<&PinOutput> {
        self.output_pins().values().next()
    }

    /// Get a hash map of all output pins.
    fn to_output_pins_list(&self) -> Vec<&PinOutput> {
        // Create a vector to put the pins in
        let mut vec = Vec::new();

        // Add each pin
        for pin in self.output_pins().values() {
            vec.push(pin);
        }

        vec
    }
}
