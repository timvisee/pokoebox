use super::{Error, LedCmd};

/// A NOP communicator, fakes communication and debugs it in the console.
pub struct Communicator;

impl super::Communicator for Communicator {
    fn send_cmd(&self, cmd: LedCmd) -> Result<(), Error> {
        debug!("NOP communicator sends: {:?}", cmd);
        Ok(())
    }
}

impl Default for Communicator {
    fn default() -> Self {
        Communicator
    }
}
