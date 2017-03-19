#![cfg(feature = "rpi")]

use error::Error;
use super::cupi::{CuPi, PinInput, PinOutput};
use super::gpio_pin_logic::GpioPinLogic;
use super::gpio_pin_config::{GpioPinConfig, IoMode};

/// A GPIO pin instance.
/// This allows you to use a GPIO pin as input or output depending on the configuration.
pub struct GpioPin {
    config: GpioPinConfig,
    input: Option<PinInput>,
    output: Option<PinOutput>,
    output_logic: GpioPinLogic
}

impl GpioPin {

    /// Construct a new GPIO pin with the given configuration.
    pub fn from(cupi: &CuPi, config: GpioPinConfig) -> Result<Self, Error> {
        // Create the CuPi pin options struct
        let options = config.as_cupi_pin_options(cupi)?;

        // Define input and output variables
        let mut input: Option<PinInput> = None;
        let mut output: Option<PinOutput> = None;

        // Create the pin
        match config.io_mode().unwrap() {
            &IoMode::Input => input = Some(options.input()),
            &IoMode::Output => {
                // Create the output pin instance
                output = Some(options.output());

                // Write the default state
                output.write(config.output_default().into_cupi());
            },
        }

        // Construct a new pin object
        Ok(GpioPin {
            config: config,
            input: input,
            output: output,
            output_logic: config.output_default()
        })
    }

    /// Check whether this pin is an input pin.
    /// True if the pin is an input pin, false if it's an output pin.
    pub fn is_input(&self) -> bool {
        match self.config.io_mode().unwrap() {
            &IoMode::Input => true,
            &IoMode::Output => false,
        }
    }

    /// Get the CuPi input pin, if this is an input pin.
    /// None is returned if this is an output pin.
    pub fn cupi_input(&self) -> Option<&PinInput> {
        self.input.as_ref()
    }

    /// Get the CuPi output pin, if this is an output pin.
    /// None is returned if this is an input pin.
    pub fn cupi_output(&self) -> Option<&PinOutput> {
        self.output.as_ref()
    }

    /// Read the value from the pin.
    /// If this is an input pin, the value is read from the physical pin.
    /// If this is an output pin, the current output value is read.
    pub fn read(&self) -> GpioPinLogic {
        if self.is_input() {
            GpioPinLogic::from_cupi(self.input.as_ref().unwrap().read())
        } else {
            self.output_logic.clone()
        }
    }

    /// Read the value from the pin as boolean.
    /// If this is an input pin, the value is read from the physical pin.
    /// If this is an output pin, the current output value is read.
    pub fn read_bool(&self) -> bool {
        self.read().into_bool()
    }

    /// Write a logical GPIO value to the pin.
    /// This only has any effect if this is an output pin. Nothing happens when this is an input pin.
    pub fn write(&mut self, logic: GpioPinLogic) {
        // Make sure this is an output pin
        if self.is_input() {
            return;
        }

        // Write the value
        self.output.as_ref().unwrap().write(logic.as_cupi());

        // Update the stored logical output value
        self.output_logic = logic;
    }

    /// Write a boolean value to the pin.
    /// This only has any effect if this is an output pin. Nothing happens when this is an input pin.
    pub fn write_bool(&mut self, logic: bool) {
        self.write(GpioPinLogic::from_bool(logic));
    }

    /// Invert the output signal of the pin.
    /// If the current logical output is `High`, it will change to `Low` and the other way around.
    /// This only has any effect if this is an output pin. Nothing happens when this is an input pin.
    pub fn write_inverse(&mut self) {
        // Get the inverted logic, and write it
        let inverted = self.output_logic.as_inverted();
        self.write(inverted);
    }
}