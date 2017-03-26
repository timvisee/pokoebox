use result::Result;

/// Peripheral that is a button.
pub trait Button {
    /// Check whether the button is pressed.
    fn is_pressed(&self) -> Option<bool>;
}
