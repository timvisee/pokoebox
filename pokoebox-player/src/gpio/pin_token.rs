/// Token of a pin.
/// Pin tokens are used to identify a pin. Each pin that is created gets it's own unique token.
/// Pins can then be requested from the GPIO manager by their token.
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct PinToken(usize);

impl PinToken {
    /// Construct a new pin token with the given token value.
    pub fn new(token: usize) -> PinToken {
        PinToken(token)
    }
}