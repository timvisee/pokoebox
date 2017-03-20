use error::Error;

/// Peripheral that is a button.
pub trait Button {
    /// Check whether the button is pressed.
    fn is_pressed(&self) -> Result<bool, Error>;
}