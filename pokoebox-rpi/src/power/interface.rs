use pokoebox_common::pipe::Pipe;

use super::adapter::{self, Adapter};
use super::{Cmd, Event};
use crate::rpi::Rpi;

/// Power interface.
pub struct Interface {
    /// Adapter to access power information.
    adapter: Box<dyn Adapter>,

    /// Events pipe.
    pub events: Pipe<Event>,
}

impl Interface {
    /// Construct new interface.
    pub fn new(rpi: &mut Rpi) -> Result<Self, Error> {
        let events = Pipe::default();
        Ok(Self {
            adapter: adapter::select_adapter(rpi, events.clone()).map_err(Error::Adapter)?,
            events,
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
