#![cfg(feature = "rpi")]

use std::sync::{Arc, RwLock};

use gpio::gpio_manager::GpioManager;
use result::Result;
use super::perif_type::PerifType;
use super::traits::button::Button;
use super::traits::perif::Perif;
use super::traits::with_inputs::WithInputs;
use super::traits::with_sig::WithSig;
use super::signal::input_gpio_toggle::InputGpioToggle;
use super::signal::sig_id::SigId;
use super::signal::traits::sig_in::SigIn;
use super::signal::traits::sig_in_toggle::SigInToggle;

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
    pub fn new(
        name: &'static str,
        pin: usize,
        gpio_manager: Arc<RwLock<GpioManager>>,
    ) -> Result<Self> {
        // Create a GPIO button signal instance, and add it to the inputs
        let sig_button = InputGpioToggle::new(
            SigId::new(SIG_BUTTON_ID),
            SIG_BUTTON_NAME,
            pin,
            gpio_manager
        )?;

        Ok(PerifGpioButton {
            name: name,
            sig_button: sig_button
        })
    }

    /// Construct a new wrapped GPIO button peripheral.
    pub fn new_wrapped(
        name: &'static str,
        pin: usize,
        gpio_manager: Arc<RwLock<GpioManager>>,
    ) -> Result<PerifType> {
        // Create a new peripheral instance
        let perif = Self::new(name, pin, gpio_manager)?;

        // Wrap and return
        Ok(PerifType::GpioButton(perif))
    }
}

/// This is a button.
impl Button for PerifGpioButton {
    /// Check whether the button is pressed.
    fn is_pressed(&self) -> Option<bool> {
        self.sig_button.state().ok()
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
