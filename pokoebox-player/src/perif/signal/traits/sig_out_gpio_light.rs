#![cfg(feature = "old-rpi")]

use super::sig_out_gpio::SigOutGpio;
use super::sig_out_light::SigOutLight;
use gpio::pin_accessor::PinAccessor;
use result::Result;

/// A peripheral output for a light that is using GPIO.
pub trait SigOutGpioLight: SigOutGpio + SigOutLight {
    /// Get the current state of the light.
    /// `true` means that the light is on, `false` means that it's off.
    fn state(&self, pin_accessor: &PinAccessor) -> Result<bool>;

    /// Set the state of the light.
    /// `true` to turn the light on, `false` to turn it off.
    fn set_state(&mut self, state: bool, pin_accessor: &mut PinAccessor) -> Result<()>;

    /// Toggle the light state.
    fn toggle(&mut self, pin_accessor: &mut PinAccessor) -> Result<()>;
}
