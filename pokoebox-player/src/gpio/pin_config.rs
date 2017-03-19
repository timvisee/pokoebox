#![cfg(feature = "rpi")]

use error::Error;
use super::cupi::{CuPi, PinOptions};
use super::pin::Pin;
use super::logic::Logic;

/// GPIO pin configuration.
/// The pin number and io mode are required. The pull mode defaults to `None`.
pub struct PinConfig {
    /// The actual pin number.
    pin: Option<usize>,

    /// Pull mode of the pin. Defaults to none.
    pull_mode: PullMode,

    /// Input/output mode of the pin.
    io_mode: Option<IoMode>,

    /// The default output state, for an output pin
    output_default: Logic,

    /// True if this pin's logic is inverted, false if not and it's normal.
    /// Setting this to true will invert all logic internally. If inversion is enabled, and a pin
    /// is set to `High` using the provided API, the physical pin logic will be `Low`, and the other
    /// way around.
    inverted: bool
}

impl PinConfig {

    /// Construct a new configuration.
    pub fn new() -> Self {
        PinConfig {
            pin: None,
            pull_mode: PullMode::None,
            io_mode: None,
            output_default: Logic::Low,
            inverted: false,
        }
    }

    /// Construct a new configuration with the given `pin` number.
    pub fn new_with_pin(pin: usize) -> Self {
        PinConfig {
            pin: Some(pin),
            pull_mode: PullMode::None,
            io_mode: None,
            output_default: Logic::Low,
            inverted: false,
        }
    }

    /// Construct a new configuration with the given `pin` number and `io_mode` mode.
    pub fn new_with_pin_and_io(pin: usize, io_mode: IoMode) -> Self {
        PinConfig {
            pin: Some(pin),
            pull_mode: PullMode::None,
            io_mode: Some(io_mode),
            output_default: Logic::Low,
            inverted: false,
        }
    }

    /// Construct a new configuration with the given properties.
    pub fn from(pin: usize, pull_mode: PullMode, io_mode: IoMode) -> Self {
        PinConfig {
            pin: Some(pin),
            pull_mode: pull_mode,
            io_mode: Some(io_mode),
            output_default: Logic::Low,
            inverted: false,
        }
    }

    /// Get the configured pin.
    pub fn pin(&self) -> Option<&usize> {
        self.pin.as_ref()
    }

    /// Set the configured pin.
    // TODO: Use Into here to automatically wrap an option
    pub fn set_pin(&mut self, pin: Option<usize>) {
        self.pin = pin
    }

    /// Get the pull mode.
    pub fn pull_mode(&self) -> &PullMode {
        &self.pull_mode
    }

    /// Set the pull mode for this pin.
    pub fn set_pull_mode(&mut self, pull_mode: PullMode) {
        self.pull_mode = pull_mode;
    }

    /// Get the configured input/output mode.
    pub fn io_mode(&self) -> Option<&IoMode> {
        self.io_mode.as_ref()
    }

    /// Set the input/output mode for this pin.
    pub fn set_io_mode(&mut self, io_mode: Option<IoMode>) {
        self.io_mode = io_mode;
    }

    /// Set the default output state to `High`.
    /// Alias for `set_output_default(GpioPinLogic::High);`
    pub fn high(&mut self) {
        self.set_output_default(Logic::High);
    }

    /// Set the default output state to `Low`.
    /// Alias for `set_output_default(GpioPinLogic::Low);`
    pub fn low(&mut self) {
        self.set_output_default(Logic::Low);
    }

    /// Get the default logical output state, if this pin is an output pin.
    /// This state is ignored when this pin is an input pin.
    pub fn output_default(&self) -> Logic {
        self.output_default.clone()
    }

    /// Set the default logical output state, if this pin is an output pin.
    /// This state is ignored when this pin is an input pin.
    pub fn set_output_default(&mut self, output_default: Logic) {
        self.output_default = output_default;
    }

    /// Invert the current inversion state.
    ///
    /// * `false`: The normal (default) state.
    /// * `true`: The inverted state.
    pub fn invert(&mut self) {
        self.inverted = !self.inverted
    }

    /// Get the inverted state for this pin.
    ///
    /// * `false`: The normal (default) state.
    /// * `true`: The inverted state.
    pub fn inverted(&self) -> bool {
        self.inverted
    }

    /// Set the inverted state of this pin.
    ///
    /// * `false`: The normal (default) state.
    /// * `true`: The inverted state.
    pub fn set_inverted(&mut self, inverted: bool) {
        self.inverted = inverted;
    }

    /// Create a CuPi pin options instance based on the current configuration.
    ///
    /// # Errors
    ///
    /// * If the pin hasn't been configured.
    /// * If the io mode hasn't been configured.
    /// * If the CuPi pin failed to create, because the pin is invalid, in use or when you don't have
    ///   the required permissions.
    // TODO: Maybe just create a new CuPi instance each time?
    pub fn as_cupi_pin_options(&self, cupi: &CuPi) -> Result<PinOptions, Error> {
        // A pin must have been configured
        if self.pin.is_none() {
            return Err(Error::new("Can't create GPIO pin, no pin configured"));
        }

        // An input/output mode must have been configured
        if self.io_mode.is_none() {
            return Err(Error::new("Can't create GPIO pin, no io mode configured"));
        }

        // Create the pin options configuration
        let result = cupi.pin(*self.pin.as_ref().unwrap());
        if result.is_err() {
            // TODO: Wrap original error here, for better information!
            return Err(Error::new("Failed to create GPIO pin"));
        }

        // Unwrap the options
        let mut options = result.unwrap();

        // Configure the pull mode
        match self.pull_mode {
            PullMode::PullUp => {
                options.pull_up();
            },
            PullMode::PullDown => {
                options.pull_down();
            },
            PullMode::None => {},
        }

        Ok(options)
    }

    /// Convert this configuration into a pin instance.
    pub fn into_pin(self, cupi: &CuPi) -> Result<Pin, Error> {
        Pin::from(cupi, self)
    }
}

/// Pull up/down mode of the pin.
pub enum PullMode {
    /// Do not pull up or down.
    None,

    /// Pull up.
    PullUp,

    /// Pull down.
    PullDown
}

/// Input/output mode of the pin.
pub enum IoMode {
    /// Input mode, to make the pin read inputs
    Input,

    /// Output mode, to make the pin write outputs
    Output
}