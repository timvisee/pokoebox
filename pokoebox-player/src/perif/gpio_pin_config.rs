#![cfg(feature = "rpi")]

use error::Error;
use super::cupi::{CuPi, PinOptions};
use super::gpio_pin_logic::GpioPinLogic;

/// GPIO pin configuration.
/// The pin number and io mode are required. The pull mode defaults to `None`.
pub struct GpioPinConfig {
    /// The actual pin number.
    pin: Option<usize>,

    /// Pull mode of the pin. Defaults to none.
    pull_mode: PullMode,

    /// Input/output mode of the pin.
    io_mode: Option<IoMode>,

    /// The default output state, for an output pin
    output_default: GpioPinLogic
}

impl GpioPinConfig {

    /// Construct a new configuration.
    pub fn new() -> Self {
        GpioPinConfig {
            pin: None,
            pull_mode: PullMode::None,
            io_mode: None,
            output_default: GpioPinLogic::Low
        }
    }

    /// Construct a new configuration with the given `pin` number.
    pub fn new_with_pin(pin: usize) -> Self {
        GpioPinConfig {
            pin: Some(pin),
            pull_mode: PullMode::None,
            io_mode: None,
            output_default: GpioPinLogic::Low
        }
    }

    /// Construct a new configuration with the given `pin` number and `io_mode` mode.
    pub fn new_with_pin_and_io(pin: usize, io_mode: IoMode) -> Self {
        GpioPinConfig {
            pin: Some(pin),
            pull_mode: PullMode::None,
            io_mode: Some(io_mode),
            output_default: GpioPinLogic::Low
        }
    }

    /// Construct a new configuration with the given properties.
    pub fn from(pin: usize, pull_mode: PullMode, io_mode: IoMode) -> Self {
        GpioPinConfig {
            pin: Some(pin),
            pull_mode: pull_mode,
            io_mode: Some(io_mode),
            output_default: GpioPinLogic::Low
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
        self.set_output_default(GpioPinLogic::High);
    }

    /// Set the default output state to `Low`.
    /// Alias for `set_output_default(GpioPinLogic::Low);`
    pub fn low(&mut self) {
        self.set_output_default(GpioPinLogic::Low);
    }

    /// Get the default logical output state, if this pin is an output pin.
    /// This state is ignored when this pin is an input pin.
    pub fn output_default(&self) -> GpioPinLogic {
        self.output_default.clone()
    }

    /// Set the default logical output state, if this pin is an output pin.
    /// This state is ignored when this pin is an input pin.
    pub fn set_output_default(&mut self, output_default: GpioPinLogic) {
        self.output_default = output_default;
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
    Input,
    Output
}