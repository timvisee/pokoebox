use super::{ButtonConfig, Error, Event};

/// A generic button adapter.
pub trait Adapter {
    /// Set-up the given button.
    fn setup_button(
        &self,
        button: ButtonConfig,
        callback: Box<dyn FnMut(Event) + Send + 'static>,
    ) -> Result<(), Error>;
}
