#![cfg(feature = "rpi")]

use std::collections::HashMap;

use error::Error;
use gpio::pin::Pin;
use gpio::pin_config::{PinConfig, IoMode};
use super::cupi::CuPi;
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
    pub fn new(id: SigId, name: &'static str, pin: usize, cupi: &CuPi) -> Result<Self, Error> {
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
        obj.setup_pins(cupi)?;

        Ok(obj)
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
    fn set_state(&mut self, state: bool) -> Result<(), Error> {
        // Get the light pin
        let result = self.gpio_pin_mut(GPIO_PIN_KEY_LIGHT);
        if result.is_none() {
            return Err(Error::new("Unable to get light pin"));
        }

        // Unwrap the pin, and set the state
        result.unwrap().write_bool(state);

        Ok(())
    }
}

impl SigOutGpioLight for OutputGpioLight {}
