use super::sig_in_toggle::SigInToggle;
use crate::result::Result;

/// A trait for a toggle signal that is generic.
pub trait SigInGenericToggle: SigInToggle {
    /// Get the state of the toggle as a boolean.
    /// When using this for a button,
    /// true might be returned when the button is pressed.
    fn state(&self) -> Result<bool>;
}
