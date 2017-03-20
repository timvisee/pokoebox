use super::sig_in::SigIn;

pub trait SigInToggle: SigIn {

    /// Get the state of the toggle as a boolean.
    /// When using this for a button, true might be returned when the button is pressed.
    fn state(&self) -> bool;
}