use super::{Error, LedCmd};

/// A generic adapter to control LEDs.
pub trait Adapter {
    /// Send command to remote controller.
    fn send_cmd(&self, cmd: LedCmd) -> Result<(), Error>;
}
