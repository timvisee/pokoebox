use gpio::gpio_manager::PinAccessor;
use result::Result;
use super::gpio::Gpio;
use super::light::Light;

/// A trait for a GPIO light peripheral.
pub trait GpioLight: Gpio + Light {
    /// Check whether the light is lit.
    fn is_lit(&self, pin_accessor: &PinAccessor) -> Result<bool>;

    /// Set whether the light is lit.
    fn set_lit(&mut self, lit: bool, pin_accessor: &mut PinAccessor) -> Result<()>;

    /// Toggle whether the light is lit.
    fn toggle_lit(&mut self, pin_accessor: &mut PinAccessor) -> Result<()>;
}