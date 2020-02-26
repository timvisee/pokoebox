use super::communicator::{self, Communicator};
use super::{Led, LedCmd};

/// LED controller.
pub struct Controller {
    /// Communicator to send LED comamnds through.
    communicator: Communicator,
}

impl Controller {
    // TODO: propagate errors
    pub fn new() -> Result<Self, Error> {
        let controller = Self {
            communicator: Communicator::new().map_err(Error::Communicator)?,
        };

        // Reset LEDs
        controller.led_reset()?;

        Ok(controller)
    }

    // TODO: propagate errors
    pub fn led_set(&self, led: Led, level: bool) -> Result<(), Error> {
        self.communicator
            .bus_send_cmd(LedCmd::LedSet(led, level))
            .map_err(Error::Communicator)
    }

    // TODO: propagate errors
    pub fn led_reset(&self) -> Result<(), Error> {
        self.communicator
            .bus_send_cmd(LedCmd::LedReset)
            .map_err(Error::Communicator)
    }
}

#[derive(Debug)]
pub enum Error {
    /// Communicator error.
    Communicator(communicator::Error),
}
