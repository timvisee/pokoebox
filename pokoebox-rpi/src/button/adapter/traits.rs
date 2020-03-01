use super::{ButtonConfig, Error, Event};

/// A generic button adapter.
pub trait Adapter: Send + Sync {
    /// Set-up the given button.
    fn setup_button(
        &self,
        button: ButtonConfig,
        callback: Box<dyn FnMut(Event) + Send + 'static>,
    ) -> Result<(), Error>;
}
