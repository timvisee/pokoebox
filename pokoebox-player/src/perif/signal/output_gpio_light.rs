#![cfg(feature = "rpi")]

use std::collections::HashMap;

use error::Error;
use gpio::gpio_manager::GpioManager;
use gpio::pin::Pin;
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
    pins: HashMap<&'static str, Pin>
}

impl OutputGpioLight {
    /// Create a new instance.
    /// The GPIO pin of the light must be passed to the `pin` parameter.
    pub fn new(id: SigId, name: &'static str, pin: usize, gpio_manager: &GpioManager) -> Result<Self> {
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
    fn find_light_pin(&self) -> Result<&Pin> {
        // Get the light pin
        let result = self.gpio_pin(GPIO_PIN_KEY_LIGHT);
        if result.is_none() {
            return Err(Error::new("Unable to get light pin"));
        }

        // Unwrap the pin, and set the state
        Ok(result.unwrap())
    }

    /// Find the GPIO pin for the light, mutable.
    fn find_light_pin_mut(&mut self) -> Result<&mut Pin> {
        // Get the light pin
        let result = self.gpio_pin_mut(GPIO_PIN_KEY_LIGHT);
        if result.is_none() {
            return Err(Error::new("Unable to get light pin"));
        }

        // Unwrap the pin, and set the state
        Ok(result.unwrap())
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

    fn gpio_pins(&self) -> &HashMap<&'static str, Pin> {
        &self.pins
    }

    fn gpio_pins_mut(&mut self) -> &mut HashMap<&'static str, Pin> {
        &mut self.pins
    }

    fn add_gpio_pin(&mut self, key: &'static str, pin: Pin) {
        self.pins.insert(key, pin);
    }
}

impl SigOut for OutputGpioLight {}

impl SigOutGpio for OutputGpioLight {}

impl SigOutLight for OutputGpioLight {
    fn state(&self) -> Result<bool> {
        Ok(self.find_light_pin()?.read_bool())
    }

    fn set_state(&mut self, state: bool) -> Result<()> {
        self.find_light_pin_mut()?.write_bool(state);
        Ok(())
    }

    fn toggle(&mut self) -> Result<()> {
        self.find_light_pin_mut()?.write_inverse();
        Ok(())
    }
}

impl SigOutGpioLight for OutputGpioLight {}
