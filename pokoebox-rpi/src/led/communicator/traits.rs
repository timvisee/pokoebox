use super::{Error, LedCmd};

/// A generic communicator to control LEDs.
pub trait Communicator {
    /// Send command to remote controller.
    fn send_cmd(&self, cmd: LedCmd) -> Result<(), Error>;
}
