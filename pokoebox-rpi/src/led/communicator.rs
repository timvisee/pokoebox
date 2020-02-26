use std::sync::Mutex;

use rppal::i2c::{self, I2c};

use super::LedCmd;

/// The address of the external LED controller.
const CONTROLLER_I2C_ADDRESS: u16 = 8;

/// Communicator. Talks to remote LED controller.
pub struct Communicator {
    /// i2c bus to send commands over.
    bus: Mutex<I2c>,
}

impl Communicator {
    // TODO: propagate error
    pub fn new() -> Result<Self, Error> {
        let mut bus = I2c::new().map_err(Error::I2c)?;
        bus.set_slave_address(CONTROLLER_I2C_ADDRESS)
            .map_err(Error::I2c)?;

        Ok(Self {
            bus: Mutex::new(bus),
        })
    }

    /// Send command to remote controller.
    pub fn bus_send_cmd(&self, cmd: LedCmd) -> Result<(), Error> {
        self.bus_send_raw(cmd.to_protocol_cmd())
    }

    fn bus_send_raw(&self, cmd: String) -> Result<(), Error> {
        // Build byte buffer to send
        let mut bytes = cmd.into_bytes();
        bytes.push(b'\n');

        // Write bytes
        let written = self
            .bus
            .lock()
            .expect("failed to obtain i2c bus lock")
            .write(&bytes)
            .map_err(Error::I2c)?;
        if written < bytes.len() {
            Err(Error::SentPartial)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub enum Error {
    /// While sending data, only part of the data was sent.
    SentPartial,

    /// An i2c bus error.
    I2c(i2c::Error),
}
