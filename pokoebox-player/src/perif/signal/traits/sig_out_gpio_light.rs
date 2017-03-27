#![cfg(feature = "rpi")]

use result::Result;
use gpio::gpio_manager::GpioManager;
use super::sig_out_gpio::SigOutGpio;
use super::sig_out_light::SigOutLight;

/// A peripheral output for a light that is using GPIO.
pub trait SigOutGpioLight: SigOutGpio + SigOutLight {
    /// Get the current state of the light.
    /// `true` means that the light is on, `false` means that it's off.
    fn state(&self, gpio_manager: &GpioManager) -> Result<bool>;

    /// Set the state of the light.
    /// `true` to turn the light on, `false` to turn it off.
    fn set_state(&mut self, state: bool, gpio_manager: &mut GpioManager) -> Result<()>;

    /// Toggle the light state.
    fn toggle(&mut self, gpio_manager: &mut GpioManager) -> Result<()>;
}
