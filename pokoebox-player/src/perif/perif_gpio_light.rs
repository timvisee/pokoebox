#![cfg(feature = "rpi")]

use super::cupi::CuPi;

use error::Error;
use super::perif_type::PerifType;
use super::traits::with_sig::WithSig;
use super::traits::with_outputs::WithOutputs;
use super::signal::sig_id::SigId;
use super::signal::traits::sig_out::SigOut;
use super::signal::output_gpio_light::OutputGpioLight;
use super::perif::Perif;

/// Signal ID of the light.
pub const SIG_LIGHT_ID: &'static str = "light";

/// Name of the light signal.
pub const SIG_LIGHT_NAME: &'static str = "Light";

/// Light peripheral implementation.
/// This can be used to toggle a light such as a LED.
pub struct PerifGpioLight {
    name: &'static str,
    outputs: Vec<Box<SigOut>>,
}

impl PerifGpioLight {
    /// Construct a new GPIO light peripheral.
    pub fn new(name: &'static str, pin: usize, cupi: &CuPi) -> Result<Self, Error> {
        // Create a vector of output signals
        let mut outputs: Vec<Box<SigOut>> = Vec::new();

        // Create a GPIO light signal instance, and add it to the outputs
        let sig_light = OutputGpioLight::new(
            SigId::new(SIG_LIGHT_ID),
            SIG_LIGHT_NAME,
            pin,
            cupi
        )?;
        outputs.push(Box::new(sig_light));

        Ok(PerifGpioLight {
            name: name,
            outputs: outputs
        })
    }

    /// Create a new wrapped GPIO light peripheral.
    pub fn new_wrapped(name: &'static str, pin: usize, cupi: &CuPi) -> Result<PerifType, Error> {
        // Create a new peripheral instance
        let perif = Self::new(name, pin, cupi)?;

        // Wrap and return
        Ok(PerifType::GpioLight(perif))
    }
}

/// This peripheral has outputs.
impl WithOutputs for PerifGpioLight {
    fn outputs(&self) -> &Vec<Box<SigOut>> {
        &self.outputs
    }
}

impl WithSig for PerifGpioLight {}

/// This is a peripheral.
impl Perif for PerifGpioLight {
    fn name(&self) -> &'static str {
        &self.name
    }
}
