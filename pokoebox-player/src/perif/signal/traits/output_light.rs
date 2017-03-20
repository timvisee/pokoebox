use error::Error;
use super::output::Output;

/// An output signal for a light.
pub trait OutputLight: Output {

    /// Set the state of the light.
    /// `true` to turn the light on, `false` to turn it off.
    fn set_state(&mut self, state: bool) -> Result<(), Error>;
}