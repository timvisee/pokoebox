use super::adapter::{self, Adapter};
use crate::rpi::Rpi;

/// Power interface.
pub struct Interface {
    /// Adapter to access power information.
    _adapter: Box<dyn Adapter>,
}

impl Interface {
    /// Construct new interface.
    pub fn new(rpi: &mut Rpi) -> Result<Self, Error> {
        Ok(Self {
            _adapter: adapter::select_adapter(rpi).map_err(Error::Adapter)?,
        })
    }
}

#[derive(Debug)]
pub enum Error {
    /// Adapter error.
    Adapter(adapter::Error),
}
