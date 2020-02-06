use super::sig_out_light::SigOutLight;
use crate::result::Result;

/// A trait for a generic light signal.
pub trait SigOutGenericLight: SigOutLight {
    /// Get the current state of the light.
    /// `true` means that the light is on, `false` means that it's off.
    fn state(&self) -> Result<bool>;

    /// Set the state of the light.
    /// `true` to turn the light on, `false` to turn it off.
    fn set_state(&mut self, state: bool) -> Result<()>;

    /// Toggle the light state.
    fn toggle(&mut self) -> Result<()>;
}
