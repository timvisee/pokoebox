#![cfg(feature = "rpi")]

use std::collections::HashMap;

use error::Error;
use gpio::pin::Pin;
use gpio::pin_config::PinConfig;
use super::cupi::CuPi;
use super::sig::Sig;

/// An input or output signal for a peripheral that uses GPIO features.
pub trait SigGpio: Sig {

    /// Get a list of GPIO pin configurations. This includes input and output pins.
    fn gpio_pin_configs(&self) -> &HashMap<&'static str, PinConfig>;

    /// Get a list of GPIO pin configurations, mutable. This includes input and output pins.
    fn gpio_pin_configs_mut(&mut self) -> &mut HashMap<&'static str, PinConfig>;

    /// Setup all the pins from the pin configurations.
    /// All pin configurations will be consumed, and this leaves the list of configurations empty.
    /// This uses the pin configurations from the `Self.gpio_pin_configs();` method.
    fn setup_pins(&mut self, cupi: &CuPi) -> Result<(), Error> {
        // Create a list of pins to add later on
        let mut pins = HashMap::new();

        // Create the result to return
        let mut result: Result<(), Error> = Ok(());

        {
            // Get the hash map of configurations
            let mut configs = self.gpio_pin_configs_mut();

            // Iterate through the list of pin configurations
            for (key, config) in configs.drain() {
                // Convert the configuration into a pin
                let pin = config.into_pin(&cupi);

                // If an error, set the result and break this loop
                if pin.is_err() {
                    result = Err(pin.err().unwrap());
                    break;
                }

                // Add the pin to the hash map
                pins.insert(key, pin.unwrap());
            }
        }

        // Add the created pins to the map
        for (key, pin) in pins.drain() {
            self.add_gpio_pin(key, pin);
        }

        result
    }

    /// Get a list of GPIO pin references. This includes both input and output pins.
    fn gpio_pins(&self) -> &HashMap<&'static str, Pin>;

    /// Get a list of GPIO pin references. This includes both input and output pins.
    fn gpio_pins_mut(&mut self) -> &mut HashMap<&'static str, Pin>;

    /// Get the GPIO pin with the given key.
    /// `None` is returned if there was no GPIO pin for the given `key`.
    fn gpio_pin(&self, key: &'static str) -> Option<&Pin> {
        self.gpio_pins().get(key)
    }

    /// Get the GPIO pin with the given key as mutable.
    /// `None` is returned if there was no GPIO pin for the given `key`.
    fn gpio_pin_mut(&mut self, key: &'static str) -> Option<&mut Pin> {
        self.gpio_pins_mut().get_mut(key)
    }

    /// Add the given `pin` with the given `key` to the hash map of pins.
    fn add_gpio_pin(&mut self, key: &'static str, pin: Pin);
}