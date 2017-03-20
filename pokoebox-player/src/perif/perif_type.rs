#[derive(Eq, PartialEq, Clone)]
pub enum PerifType {

    /// GPIO Light.
    #[cfg(feature = "rpi")]
    GpioLight,

    /// GPIO Button.
    #[cfg(feature = "rpi")]
    GpioButton,
}
