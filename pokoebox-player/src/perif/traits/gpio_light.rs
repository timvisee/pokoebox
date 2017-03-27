use gpio::gpio_manager::GpioManager;
use result::Result;
use super::gpio::Gpio;
use super::light::Light;

/// A trait for a GPIO light peripheral.
pub trait GpioLight: Gpio + Light {
    /// Check whether the light is lit.
    fn is_lit(&self, gpio_manager: &GpioManager) -> Result<bool>;

    /// Set whether the light is lit.
    fn set_lit(&mut self, lit: bool, gpio_manager: &mut GpioManager) -> Result<()>;

    /// Toggle whether the light is lit.
    fn toggle_lit(&mut self, gpio_manager: &mut GpioManager) -> Result<()>;
}