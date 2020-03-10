use super::{Cmd, Error};

/// A generic power adapter.
pub trait Adapter: Send + Sync {
    /// Send command to remote controller.
    fn send_cmd(&self, cmd: Cmd) -> Result<(), Error>;
}
