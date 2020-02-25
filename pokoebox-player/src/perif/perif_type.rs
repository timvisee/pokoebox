#[cfg(feature = "old-rpi")]
use super::perif_gpio_button::PerifGpioButton;
#[cfg(feature = "old-rpi")]
use super::perif_gpio_light::PerifGpioLight;

pub enum PerifType {
    /// GPIO Light.
    #[cfg(feature = "old-rpi")]
    GpioLight(PerifGpioLight),

    /// GPIO Button.
    #[cfg(feature = "old-rpi")]
    GpioButton(PerifGpioButton),
}
