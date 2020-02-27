use super::{ButtonConfig, Error, Event};

/// A NOP adapter, fakes communication and debugs it in the console.
pub struct Adapter;

impl super::Adapter for Adapter {
    fn setup_button(
        &self,
        button: ButtonConfig,
        _callback: Box<dyn FnMut(Event) + Send + 'static>,
    ) -> Result<(), Error> {
        debug!("NOP adapter set up button: {:?}", button);
        Ok(())
    }
}

impl Default for Adapter {
    fn default() -> Self {
        Adapter
    }
}
