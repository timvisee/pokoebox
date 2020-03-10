use std::sync::{Arc, Mutex};

use rppal::i2c::I2c;

use super::{Error, LedCmd};
use crate::rpi::Rpi;

/// The address of the external LED controller.
const CONTROLLER_I2C_ADDRESS: u16 = 8;

/// Adapter. Talks to remote LED controller.
pub struct Adapter {
    /// i2c bus to send commands over.
    bus: Arc<Mutex<I2c>>,
}

impl Adapter {
    /// Construct new adapter.
    pub fn new(rpi: &mut Rpi) -> Result<Self, Error> {
        Ok(Self { bus: rpi.get_i2c() })
    }

    /// Send raw data over the bus.
    fn bus_send_raw(&self, cmd: String) -> Result<(), Error> {
        // Get bus lock, set slave address
        let mut bus = self.bus.lock().expect("failed to obtain i2c bus lock");
        bus.set_slave_address(CONTROLLER_I2C_ADDRESS)
            .map_err(|_| Error::Adapter)?;

        // Build byte buffer to send
        let mut bytes = cmd.into_bytes();
        bytes.push(b'\n');

        // Write bytes
        let written = bus.write(&bytes).map_err(|_| Error::Adapter)?;
        if written < bytes.len() {
            Err(Error::Adapter)
        } else {
            Ok(())
        }
    }
}

impl super::Adapter for Adapter {
    fn send_cmd(&self, cmd: LedCmd) -> Result<(), Error> {
        self.bus_send_raw(led_cmd_to_protocol(cmd))
    }
}

/// Create protcol data for given LED command.
fn led_cmd_to_protocol(cmd: LedCmd) -> String {
    match cmd {
        LedCmd::LedReset => "led reset".into(),
        LedCmd::LedSet(led, level) => format!("led {} {}", led as u8, level as u8),
    }
}

// #[derive(Debug)]
// pub enum Error {
//     /// While sending data, only part of the data was sent.
//     SentPartial,

//     /// An i2c bus error.
//     I2c(i2c::Error),
// }
