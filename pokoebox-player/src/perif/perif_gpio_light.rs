use error::Error;
use super::traits::with_io::WithIo;
use super::traits::with_outputs::WithOutputs;
use super::signal::traits::output::Output;
use super::perif::Perif;

/// Light peripheral implementation.
/// This can be used to toggle a light such as a LED.
pub struct PerifGpioLight {
    name: &'static str,
    outputs: Vec<Box<Output>>,
}

impl PerifGpioLight {
    /// Construct a new GPIO light peripheral.
    // TODO: Configure the GPIO pin here!
    pub fn new(name: &'static str) -> Self {
        // TODO: Set up the light output!
        PerifGpioLight {
            name: name,
            outputs: Vec::new(),
        }
    }
}

/// This is a peripheral.
impl Perif for PerifGpioLight {
    fn name(&self) -> &'static str {
        self.name
    }

    fn setup(&self) -> Result<(), Error> {
        Ok(())
    }

    fn is_setup(&self) -> bool {
        true
    }
}

impl WithIo for PerifGpioLight {}

/// This peripheral has outputs.
impl WithOutputs for PerifGpioLight {
    fn outputs(&self) -> &Vec<Box<Output>> {
        &self.outputs
    }
}