use super::adapter::{self, Adapter};
use super::{Led, LedCmd};
use crate::rpi::Rpi;

/// LED interface.
pub struct Interface {
    /// Adapter to send LED comamnds through.
    adapter: Box<dyn Adapter>,
}

impl Interface {
    // TODO: propagate errors
    pub fn new(rpi: &mut Rpi) -> Result<Self, Error> {
        let interface = Self {
            adapter: adapter::select_adapter(rpi).map_err(Error::Adapter)?,
        };

        // Reset LEDs
        interface.led_reset()?;

        Ok(interface)
    }

    // TODO: propagate errors
    pub fn led_set(&self, led: Led, level: bool) -> Result<(), Error> {
        self.adapter
            .send_cmd(LedCmd::LedSet(led, level))
            .map_err(Error::Adapter)
    }

    // TODO: propagate errors
    pub fn led_reset(&self) -> Result<(), Error> {
        self.adapter
            .send_cmd(LedCmd::LedReset)
            .map_err(Error::Adapter)
    }
}

#[derive(Debug)]
pub enum Error {
    /// Adapter error.
    Adapter(adapter::Error),
}
