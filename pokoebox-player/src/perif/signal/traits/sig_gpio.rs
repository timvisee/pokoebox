#![cfg(feature = "rpi")]

use std::collections::HashMap;

use gpio::gpio_manager::{GpioManager, PinAccessor};
use gpio::pin::Pin;
use gpio::pin_token::PinToken;
use gpio::pin_config::PinConfig;
use result::Result;
use super::sig::Sig;

/// An input or output signal for a peripheral that uses GPIO features.
pub trait SigGpio: Sig {
    /// Get a list of GPIO pin configurations.
    /// This includes input and output pins.
    fn gpio_pin_configs(&self) -> &HashMap<&'static str, PinConfig>;

    /// Get a list of GPIO pin configurations, mutable.
    /// This includes input and output pins.
    fn gpio_pin_configs_mut(&mut self)
       -> &mut HashMap<&'static str, PinConfig>;

    /// Setup all the pins from the pin configurations.
    /// All pin configurations will be consumed, and this leaves the list
    /// of configurations empty.
    /// This uses the pin configurations from the `Self.gpio_pin_configs();`
    /// method.
    fn setup_pins(&mut self, gpio_manager: &mut GpioManager) -> Result<()> {
        // Create a pin tokens list of the pins that are created
        let mut pin_tokens = HashMap::new();

        // TODO: Can we un-scope this part below?
        {
            // Get the hash map of configurations
            let mut configs = self.gpio_pin_configs_mut();

            // Iterate through the list of pin configurations
            for (key, config) in configs.drain() {
                // Convert the configuration into a pin
                let token = config.into_pin(gpio_manager)?;

                // Add the pin to the hash map
                pin_tokens.insert(key, token);
            }
        }

        // Add the pin tokens of the created pins to the map
        for (key, token) in pin_tokens.drain() {
            self.add_gpio_pin_token(key, token);
        }

        Ok(())
    }

    /// Get a list of GPIO pin tokens.
    /// This includes tokens for both input and output pins.
    fn gpio_pin_tokens(&self) -> &HashMap<&'static str, PinToken>;

    /// Get the pin token for the given pin key.
    /// `None` is returned if there was no GPIO pin for the given `key`.
    fn gpio_pin_token(&self, key: &'static str) -> Option<PinToken> {
        self.gpio_pin_tokens().get(key).and_then(|token| Some(*token))
    }

    /// Get the GPIO pin with the given key.
    /// `None` is returned if there was no GPIO pin for the given `key`.
    fn gpio_pin<'a, 'b: 'a>(&'a self, key: &'static str, pin_accessor: &'b PinAccessor) -> Option<&'a Pin> {
        // Get the pin token
        let token = self.gpio_pin_token(key);
        if token.is_none() {
            return None;
        }

        // Get the pin instance by it's token
        pin_accessor.pin(token.unwrap())
    }

    /// Get the GPIO pin with the given key as mutable.
    /// `None` is returned if there was no GPIO pin for the given `key`.
    fn gpio_pin_mut<'a, 'b: 'a>(&'a mut self, key: &'static str, pin_accessor: &'b mut PinAccessor)
        -> Option<&'a mut Pin>
    {
        // Get the pin token
        let token = self.gpio_pin_token(key);
        if token.is_none() {
            return None;
        }

        // Get the pin instance by it's token
        pin_accessor.pin_mut(token.unwrap())
    }

    /// Add the given `pin` with the given `key` to the hash map of pins.
    fn add_gpio_pin_token(&mut self, key: &'static str, pin_token: PinToken);
}
