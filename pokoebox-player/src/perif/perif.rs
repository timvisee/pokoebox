use error::Error;

/// Peripheral trait, must be implemented on a peripheral struct.
/// A peripheral might be an external button or dial that is attached to the machine running
/// the application. These peripherals can play/pause music and control things like volume.
pub trait Perif {

    /// Get the name of the peripheral.
    fn name(&self) -> &'static str;

    /// Set up the peripheral to make it ready to use.
    /// When this is an GPIO peripheral, the communication pins that are used will be configured.
    fn setup(&self) -> Result<(), Error>;

    /// Check whether this peripheral has been set up, and that it is ready to use.
    ///
    /// Returns true if it's ready to use, false if not.
    fn is_setup(&self) -> bool;
}