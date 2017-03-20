use error::Error;
use super::sig_out::SigOut;

/// An output signal for a light.
pub trait SigOutLight: SigOut {

    /// Get the current state of the light.
    /// `true` means that the light is on, `false` means that it's off.
    fn state(&self) -> Result<bool, Error>;

    /// Set the state of the light.
    /// `true` to turn the light on, `false` to turn it off.
    fn set_state(&mut self, state: bool) -> Result<(), Error>;

    /// Toggle the light state.
    fn toggle(&mut self) -> Result<(), Error>;
}