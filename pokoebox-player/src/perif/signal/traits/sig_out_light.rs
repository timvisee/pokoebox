use error::Error;
use super::sig_out::SigOut;

/// An output signal for a light.
pub trait SigOutLight: SigOut {

    /// Set the state of the light.
    /// `true` to turn the light on, `false` to turn it off.
    fn set_state(&mut self, state: bool) -> Result<(), Error>;
}