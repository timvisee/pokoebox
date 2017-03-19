#![cfg(feature = "rpi")]

use error::Error;
use super::cupi::{CuPi, PinInput, PinOutput};
use super::gpio_pin_config::{GpioPinConfig, IoMode};

/// A GPIO pin instance.
/// This allows you to use a GPIO pin as input or output depending on the configuration.
pub struct GpioPin {
    config: GpioPinConfig,
    input: Option<PinInput>,
    output: Option<PinOutput>
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
            &IoMode::Output => output = Some(options.output()),
        }

        // Construct a new pin object
        Ok(GpioPin {
            config: config,
            input: input,
            output: output,
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
}