use super::generic::Generic;
use super::light::Light;
use crate::result::Result;

/// A trait for a generic light peripheral.
pub trait GenericLight: Generic + Light {
    /// Check whether the light is lit.
    fn is_lit(&self) -> Option<bool>;

    /// Set whether the light is lit.
    fn set_lit(&mut self) -> Result<()>;

    /// Toggle whether the light is lit.
    fn toggle_lit(&mut self) -> Result<()>;
}
