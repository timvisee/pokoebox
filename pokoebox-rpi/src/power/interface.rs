use super::adapter::{self, Adapter};
use super::Cmd;
use crate::rpi::Rpi;

/// Power interface.
pub struct Interface {
    /// Adapter to access power information.
    adapter: Box<dyn Adapter>,
}

impl Interface {
    /// Construct new interface.
    pub fn new(rpi: &mut Rpi) -> Result<Self, Error> {
        Ok(Self {
            adapter: adapter::select_adapter(rpi).map_err(Error::Adapter)?,
        })
    }

    pub fn send_cmd(&self, cmd: Cmd) -> Result<(), adapter::Error> {
        self.adapter.send_cmd(cmd)
    }
}

#[derive(Debug)]
pub enum Error {
    /// Adapter error.
    Adapter(adapter::Error),
}
