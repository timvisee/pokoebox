use super::adapter::{self, Adapter};
use super::{ButtonConfig, Event};

/// Button interface.
pub struct Interface {
    /// Adapter to access buttons.
    adapter: Box<dyn Adapter>,
}

impl Interface {
    /// Construct new interface.
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            adapter: adapter::select_adapter().map_err(Error::Adapter)?,
        })
    }

    /// Set up the given button configuration.
    pub fn setup_button<C>(&self, config: ButtonConfig, callback: C) -> Result<(), Error>
    where
        C: FnMut(Event) + Send + 'static,
    {
        self.adapter
            .setup_button(config, Box::new(callback))
            .map_err(Error::Adapter)
    }
}

#[derive(Debug)]
pub enum Error {
    /// Adapter error.
    Adapter(adapter::Error),
}
