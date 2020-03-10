use super::adapter::{self, Adapter};

/// Power interface.
pub struct Interface {
    /// Adapter to access power information.
    _adapter: Box<dyn Adapter>,
}

impl Interface {
    /// Construct new interface.
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            _adapter: adapter::select_adapter().map_err(Error::Adapter)?,
        })
    }
}

#[derive(Debug)]
pub enum Error {
    /// Adapter error.
    Adapter(adapter::Error),
}
