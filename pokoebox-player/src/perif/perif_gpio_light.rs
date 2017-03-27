#![cfg(feature = "rpi")]

use gpio::gpio_manager::GpioManager;
use result::Result;
use super::perif_type::PerifType;
use super::traits::perif::Perif;
use super::traits::with_sig::WithSig;
use super::traits::with_outputs::WithOutputs;
use super::signal::sig_id::SigId;
use super::signal::traits::sig_out::SigOut;
use super::signal::traits::sig_out_light::SigOutLight;
use super::signal::output_gpio_light::OutputGpioLight;

/// Signal ID of the light.
pub const SIG_LIGHT_ID: &'static str = "light";

/// Name of the light signal.
pub const SIG_LIGHT_NAME: &'static str = "Light";

/// Light peripheral implementation.
/// This can be used to toggle a light such as a LED.
pub struct PerifGpioLight {
    name: &'static str,
    sig_light: OutputGpioLight
}

impl PerifGpioLight {
    /// Construct a new GPIO light peripheral.
    pub fn new(
        name: &'static str,
        pin: usize,
        gpio_manager: &mut GpioManager
    ) -> Result<Self> {
        // Create a GPIO light signal instance, and add it to the outputs
        let sig_light = OutputGpioLight::new(
            SigId::new(SIG_LIGHT_ID),
            SIG_LIGHT_NAME,
            pin,
            gpio_manager
        )?;

        Ok(PerifGpioLight {
            name: name,
            sig_light: sig_light
        })
    }

    /// Create a new wrapped GPIO light peripheral.
    pub fn new_wrapped(
        name: &'static str,
        pin: usize,
        gpio_manager: &mut GpioManager
    ) -> Result<PerifType> {
        // Create a new peripheral instance
        let perif = Self::new(name, pin, gpio_manager)?;

        // Wrap and return
        Ok(PerifType::GpioLight(perif))
    }

    /// Get the current state of the light.
    pub fn state(&self) -> Result<bool> {
        self.sig_light.state()
    }

    /// Set the state of the light.
    pub fn set_state(&mut self, state: bool) -> Result<()> {
        self.sig_light.set_state(state)
    }

    /// Toggle the light.
    pub fn toggle(&mut self) -> Result<()> {
        self.sig_light.toggle()
    }
}

/// This peripheral has outputs.
impl WithOutputs for PerifGpioLight {
    fn list_outputs(&self) -> Vec<&SigOut> {
        vec![
            &self.sig_light
        ]
    }
}

impl WithSig for PerifGpioLight {}

/// This is a peripheral.
impl Perif for PerifGpioLight {
    fn name(&self) -> &'static str {
        &self.name
    }
}
