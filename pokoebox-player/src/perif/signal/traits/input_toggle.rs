use super::input::Input;

pub trait InputToggle: Input {

    /// Get the state of the toggle as a boolean.
    /// When using this for a button, true might be returned when the button is pressed.
    fn state(&self) -> bool;
}