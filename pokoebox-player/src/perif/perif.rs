use super::perif_type::PerifType;

/// Peripheral trait, must be implemented on a peripheral struct.
/// A peripheral might be an external button or dial that is attached to the machine running
/// the application. These peripherals can play/pause music and control things like volume.
pub trait Perif {

    /// Get the name of the peripheral.
    fn name(&self) -> &'static str;

    /// Get the peripheral type.
    fn perif_type(&self) -> PerifType;
}