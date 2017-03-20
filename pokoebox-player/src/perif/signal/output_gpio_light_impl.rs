#![cfg(feature = "rpi")]

use std::collections::HashMap;

use error::Error;
use gpio::pin::Pin;
use gpio::pin_config::{PinConfig, IoMode};
use super::cupi::CuPi;
use super::io_id::IoId;
use super::traits::io::Io;
use super::traits::io_gpio::IoGpio;
use super::traits::output::Output;
use super::traits::output_gpio::OutputGpio;
use super::traits::output_gpio_light::OutputGpioLight;
use super::traits::output_light::OutputLight;

/// Key for the GPIO pin of the light
pub const GPIO_PIN_KEY_LIGHT: &'static str = "light";

/// Output signal for a peripheral to control a light.
pub struct OutputGpioLightImpl {
    id: IoId,
    name: &'static str,
    pin_configs: HashMap<&'static str, PinConfig>,
    pins: HashMap<&'static str, Pin>
}

impl OutputGpioLightImpl {
    /// Create a new instance.
    /// The GPIO pin of the light must be passed to the `pin` parameter.
    pub fn new(id: IoId, name: &'static str, pin: usize, cupi: &CuPi) -> Result<Self, Error> {
        // Create a hash map of pin configurations
        let mut pin_configs = HashMap::new();

        // Create the pin configuration, and add it to the configurations list
        pin_configs.insert(
            GPIO_PIN_KEY_LIGHT,
            PinConfig::new_with_pin_and_io(pin, IoMode::Output)
        );

        // Construct the object
        let mut obj = OutputGpioLightImpl {
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

impl Io for OutputGpioLightImpl {
    fn id(&self) -> &IoId {
        &self.id
    }

    fn name(&self) -> &'static str {
        &self.name
    }
}

impl IoGpio for OutputGpioLightImpl {
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

impl Output for OutputGpioLightImpl {}

impl OutputGpio for OutputGpioLightImpl {}

impl OutputLight for OutputGpioLightImpl {
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

impl OutputGpioLight for OutputGpioLightImpl {}
