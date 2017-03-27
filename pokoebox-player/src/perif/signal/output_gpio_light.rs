#![cfg(feature = "rpi")]

use std::collections::HashMap;

use error::Error;
use gpio::gpio_manager::GpioManager;
use gpio::pin::Pin;
use gpio::pin_token::PinToken;
use gpio::pin_config::{PinConfig, IoMode};
use result::Result;
use super::sig_id::SigId;
use super::traits::sig::Sig;
use super::traits::sig_gpio::SigGpio;
use super::traits::sig_out::SigOut;
use super::traits::sig_out_gpio::SigOutGpio;
use super::traits::sig_out_gpio_light::SigOutGpioLight;
use super::traits::sig_out_light::SigOutLight;

/// Key for the GPIO pin of the light
pub const GPIO_PIN_KEY_LIGHT: &'static str = "light";

/// Output signal for a peripheral to control a light.
pub struct OutputGpioLight {
    id: SigId,
    name: &'static str,
    pin_configs: HashMap<&'static str, PinConfig>,
    pins: HashMap<&'static str, PinToken>,
}

impl OutputGpioLight {
    /// Create a new instance.
    /// The GPIO pin of the light must be passed to the `pin` parameter.
    pub fn new(
        id: SigId, 
        name: &'static str,
        pin: usize,
        gpio_manager: &mut GpioManager
    ) -> Result<Self> {
        // Create a hash map of pin configurations
        let mut pin_configs = HashMap::new();

        // Create the pin configuration, and add it to the configurations list
        pin_configs.insert(
            GPIO_PIN_KEY_LIGHT,
            PinConfig::new_with_pin_and_io(pin, IoMode::Output)
        );

        // Construct the object
        let mut obj = OutputGpioLight {
            id: id,
            name: name,
            pin_configs: pin_configs,
            pins: HashMap::new(),
        };

        // Setup the pins
        obj.setup_pins(gpio_manager)?;

        Ok(obj)
    }

    /// Find the GPIO pin for the light.
    fn find_light_pin(&self) -> Option<&Pin> {
        self.gpio_pin(GPIO_PIN_KEY_LIGHT)
    }

    /// Find the GPIO pin for the light, mutable.
    fn find_light_pin_mut(&self) -> Option<&mut Pin> {
        self.gpio_pin_mut(GPIO_PIN_KEY_LIGHT)
    }
}

impl Sig for OutputGpioLight {
    fn id(&self) -> &SigId {
        &self.id
    }

    fn name(&self) -> &'static str {
        &self.name
    }
}

impl SigGpio for OutputGpioLight {
    fn gpio_pin_configs(&self) -> &HashMap<&'static str, PinConfig> {
        &self.pin_configs
    }

    fn gpio_pin_configs_mut(&mut self) -> &mut HashMap<&'static str, PinConfig> {
        &mut self.pin_configs
    }

    fn gpio_pin_tokens(&self) -> &HashMap<&'static str, PinToken> {
        &self.pins
    }

    fn add_gpio_pin_token(&mut self, key: &'static str, pin_token: PinToken) {
        self.pins.insert(key, pin_token);
    }
}

impl SigOut for OutputGpioLight {}

impl SigOutGpio for OutputGpioLight {}

impl SigOutLight for OutputGpioLight {
    fn state(&self) -> Result<bool> {
        self.find_light_pin().and_then(|pin| Some(pin.read_bool())).ok_or(
            Error::new("Failed to read light state, unable to find light pin.")
        )
    }

    fn set_state(&mut self, state: bool) -> Result<()> {
        // Get the light pin
        let pin = self.find_light_pin_mut().ok_or(
            Error::new("Failed to toggle light, unable to find light pin.")
        )?;

        // Write the state
        pin.write_bool(state);
        Ok(())
    }

    fn toggle(&mut self) -> Result<()> {
        // Get the light pin
        let pin = self.find_light_pin_mut().ok_or(
            Error::new("Failed to toggle light, unable to find light pin.")
        )?;

        // Write the inverse
        pin.write_inverse();
        Ok(())
    }
}

impl SigOutGpioLight for OutputGpioLight {}
