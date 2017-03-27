use super::button::Button;
use super::generic::Generic;

/// A trait for a generic button peripheral.
pub trait GenericButton: Generic + Button {
    /// Check whether the button is pressed.
    fn is_pressed(&self) -> Option<bool>;
}