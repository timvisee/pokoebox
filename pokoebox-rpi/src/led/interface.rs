use super::communicator::{self, Communicator};
use super::{Led, LedCmd};

/// LED interface.
pub struct Interface {
    /// Communicator to send LED comamnds through.
    communicator: Box<dyn Communicator>,
}

impl Interface {
    // TODO: propagate errors
    pub fn new() -> Result<Self, Error> {
        let interface = Self {
            communicator: communicator::select_communicator().map_err(Error::Communicator)?,
        };

        // Reset LEDs
        interface.led_reset()?;

        Ok(interface)
    }

    // TODO: propagate errors
    pub fn led_set(&self, led: Led, level: bool) -> Result<(), Error> {
        self.communicator
            .send_cmd(LedCmd::LedSet(led, level))
            .map_err(Error::Communicator)
    }

    // TODO: propagate errors
    pub fn led_reset(&self) -> Result<(), Error> {
        self.communicator
            .send_cmd(LedCmd::LedReset)
            .map_err(Error::Communicator)
    }
}

#[derive(Debug)]
pub enum Error {
    /// Communicator error.
    Communicator(communicator::Error),
}
