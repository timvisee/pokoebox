use super::{Cmd, Error};

/// A NOP adapter, fakes communication and debugs it in the console.
pub struct Adapter;

impl super::Adapter for Adapter {
    fn send_cmd(&self, cmd: Cmd) -> Result<(), Error> {
        debug!("NOP adapter sends: {:?}", cmd);
        Ok(())
    }
}

impl Default for Adapter {
    fn default() -> Self {
        Adapter
    }
}
