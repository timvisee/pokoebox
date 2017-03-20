#[cfg(feature = "rpi")]
use super::perif_gpio_button::PerifGpioButton;
#[cfg(feature = "rpi")]
use super::perif_gpio_light::PerifGpioLight;

pub enum PerifType {

    /// GPIO Light.
    #[cfg(feature = "rpi")]
    GpioLight(PerifGpioLight),

    /// GPIO Button.
    #[cfg(feature = "rpi")]
    GpioButton(PerifGpioButton),
}
