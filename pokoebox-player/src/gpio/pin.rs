#![cfg(feature = "rpi")]

use result::Result;
use super::cupi::{CuPi, PinInput, PinOutput};
use super::gpio_manager::GpioManager;
use super::logic::Logic;
use super::pin_config::{PinConfig, IoMode};

/// A GPIO pin instance.
/// This allows you to use a GPIO pin as input or output depending on the
/// configuration.
pub struct Pin {
    config: PinConfig,
    input: Option<PinInput>,
    output: Option<PinOutput>,
    output_logic: Logic
}

impl Pin {

    /// Construct a new GPIO pin with the given configuration.
    pub fn from(gpio_manager: &GpioManager, config: PinConfig) -> Result<Self> {
        // Create the CuPi pin options struct
        let options = config.as_cupi_pin_options(gpio_manager.cupi())?;

        // Define input and output variables
        let mut input: Option<PinInput> = None;
        let mut output: Option<PinOutput> = None;
        let output_logic = config.output_default();

        // Create the pin
        match config.io_mode().unwrap() {
            &IoMode::Input => input = Some(options.input()),
            &IoMode::Output => {
                // Get the CuPi pin output instance
                let pin_output = options.output();

                // Write the default state
                pin_output.write(config.output_default().into_cupi());

                // Create the output pin instance
                output = Some(pin_output);
            },
        }

        // Construct a new pin object
        Ok(Pin {
            config: config,
            input: input,
            output: output,
            output_logic: output_logic
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
    pub fn read(&self) -> Logic {
        // Return the stored output value if this is an output pin
        if !self.is_input() {
            return self.output_logic.clone();
        }

        // Read the physical value
        let mut phys_logic = Logic::from_cupi(
            self.input.as_ref().unwrap().read()
        );

        // Invert the physical logic if configured
        if self.config.inverted() {
            phys_logic = phys_logic.into_inverted();
        }

        // Return the value
        phys_logic
    }

    /// Read the value from the pin as boolean.
    /// If this is an input pin, the value is read from the physical pin.
    /// If this is an output pin, the current output value is read.
    pub fn read_bool(&self) -> bool {
        self.read().into_bool()
    }

    /// Write a logical GPIO value to the pin.
    /// This only has any effect if this is an output pin.
    /// Nothing happens when this is an input pin.
    pub fn write(&mut self, logic: Logic) {
        // Make sure this is an output pin
        if self.is_input() {
            return;
        }

        // Get the physical value to write, and invert it if configured
        let mut phys_logic = logic.clone();
        if self.config.inverted() {
            phys_logic = phys_logic.into_inverted();
        }

        // Write the physical value
        self.output.as_ref().unwrap().write(phys_logic.into_cupi());

        // Update the stored logical output value
        self.output_logic = logic;
    }

    /// Write a boolean value to the pin.
    /// This only has any effect if this is an output pin.
    /// Nothing happens when this is an input pin.
    pub fn write_bool(&mut self, logic: bool) {
        self.write(Logic::from_bool(logic));
    }

    /// Invert the output signal of the pin.
    /// If the current logical output is `High`,
    /// it will change to `Low` and the other way around.
    /// This only has any effect if this is an output pin.
    /// Nothing happens when this is an input pin.
    pub fn write_inverse(&mut self) {
        // Get the inverted logic, and write it
        let inverted = self.output_logic.as_inverted();
        self.write(inverted);
    }

    /// Write the `High` state.
    /// This only has any effect if this is an output pin.
    /// Nothing happens when this is an input pin.
    pub fn high(&mut self) {
        self.write(Logic::High);
    }

    /// Write the `Low` state.
    /// This only has any effect if this is an output pin.
    /// Nothing happens when this is an input pin.
    pub fn low(&mut self) {
        self.write(Logic::Low);
    }
}
