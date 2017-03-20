#![cfg(feature = "rpi")]

use super::cupi::CuPi;

use error::Error;
use super::perif_type::PerifType;
use super::traits::with_inputs::WithInputs;
use super::traits::with_sig::WithSig;
use super::signal::input_gpio_toggle::InputGpioToggle;
use super::signal::sig_id::SigId;
use super::signal::traits::sig_in::SigIn;
use super::perif::Perif;

/// Signal ID of the button.
pub const SIG_BUTTON_ID: &'static str = "button";

/// Name of the button signal.
pub const SIG_BUTTON_NAME: &'static str = "Button";

/// Button peripheral implementation.
/// This can be used to bind actions to a button press.
pub struct PerifGpioButton {
    name: &'static str,
    sig_button: InputGpioToggle,
}

impl PerifGpioButton {
    /// Construct a new GPIO button peripheral.
    pub fn new(name: &'static str, pin: usize, cupi: &CuPi) -> Result<Self, Error> {
        // Create a GPIO button signal instance, and add it to the inputs
        let sig_button = InputGpioToggle::new(
            SigId::new(SIG_BUTTON_ID),
            SIG_BUTTON_NAME,
            pin,
            cupi
        )?;

        Ok(PerifGpioButton {
            name: name,
            sig_button: sig_button
        })
    }

    /// Construct a new wrapped GPIO button peripheral.
    pub fn new_wrapped(name: &'static str, pin: usize, cupi: &CuPi) -> Result<PerifType, Error> {
        // Create a new peripheral instance
        let perif = Self::new(name, pin, cupi)?;

        // Wrap and return
        Ok(PerifType::GpioButton(perif))
    }
}

/// This peripheral has inputs.
impl WithInputs for PerifGpioButton {
    fn list_inputs(&self) -> Vec<&SigIn> {
        vec![
            &self.sig_button
        ]
    }
}

impl WithSig for PerifGpioButton {}

/// This is a peripheral.
impl Perif for PerifGpioButton {
    fn name(&self) -> &'static str {
        &self.name
    }
}
