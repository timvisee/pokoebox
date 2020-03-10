use std::sync::{Arc, Mutex};

use bytes::{Buf, Bytes};
use pokoebox_common::pipe::Pipe;
use rppal::i2c::I2c;

use super::{Cmd, Error, Event};
use crate::rpi::Rpi;

/// The address of the external INA260 controller.
const CONTROLLER_I2C_ADDRESS: u16 = 0x40;

/// INA260 configuration register.
#[allow(unused)]
const INA260_REG_CONFIG: u8 = 0x00;

/// INA260 current measurement register (signed, 1.25 mA)
const INA260_REG_CURRENT: u8 = 0x01;

/// INA260 bus voltage register (1.25 mV)
const INA260_REG_BUSVOLTAGE: u8 = 0x02;

/// INA260 power register.
const INA260_REG_POWER: u8 = 0x03;

/// INA260 mask enable register.
#[allow(unused)]
const INA260_REG_MASK_ENABLE: u8 = 0x06;

/// INA260 alert limit register.
#[allow(unused)]
const INA260_REG_ALERT_LIMIT: u8 = 0x07;

/// INA260 manufacturer ID register.
#[allow(unused)]
const INA260_REG_MFG_UID: u8 = 0xFE;

/// INA260 die ID and revision register.
#[allow(unused)]
const INA260_REG_DIE_UID: u8 = 0xFF;

/// Adapter. Talks to remote controller.
pub struct Adapter {
    /// i2c bus to send commands over.
    bus: Arc<Mutex<I2c>>,

    /// Events pipe.
    events: Pipe<Event>,
}

impl Adapter {
    /// Construct new adapter.
    pub fn new(rpi: &mut Rpi, events: Pipe<Event>) -> Result<Self, Error> {
        Ok(Self {
            bus: rpi.get_i2c(),
            events,
        })
    }

    /// Read raw data from controller registry.
    fn bus_read_raw(&self, reg: u8) -> Result<Bytes, Error> {
        // Get bus lock, set slave address
        let mut bus = self.bus.lock().expect("failed to obtain i2c bus lock");
        bus.set_slave_address(CONTROLLER_I2C_ADDRESS)
            .map_err(|_| Error::Adapter)?;

        // Allocate output buffer, write and read
        let mut buff = vec![0; 2];
        bus.write_read(&[reg], &mut buff)
            .map_err(|_| Error::Adapter)?;

        Ok(buff.into())
    }

    /// Read current in amperes (signed).
    fn bus_read_current(&self) -> Result<f32, Error> {
        Ok(self.bus_read_raw(INA260_REG_CURRENT)?.get_i16() as f32 * 1.25 * 1000.0)
    }

    /// Read bus voltage in volts.
    fn bus_read_bus_voltage(&self) -> Result<f32, Error> {
        Ok(self.bus_read_raw(INA260_REG_BUSVOLTAGE)?.get_i16() as f32 * 1.25 * 1000.0)
    }

    /// Read power in watts (unsigned).
    fn bus_read_power(&self) -> Result<f32, Error> {
        Ok(self.bus_read_raw(INA260_REG_POWER)?.get_i16() as f32 * 1.25 * 1.25 * 1000.0)
    }
}

impl super::Adapter for Adapter {
    fn send_cmd(&self, cmd: Cmd) -> Result<(), Error> {
        match cmd {
            Cmd::Poll => {
                // Read power values, emit event
                let (current, voltage, power) = (
                    self.bus_read_current()?,
                    self.bus_read_bus_voltage()?,
                    self.bus_read_power()?,
                );
                if let Err(err) = self.events.send(Event::Power(current, voltage, power)) {
                    error!(
                        "Failed to read power state from power controller: {:?}",
                        err
                    );
                }
            }
        }

        Ok(())
    }
}

// #[derive(Debug)]
// pub enum Error {
//     /// While sending data, only part of the data was sent.
//     SentPartial,

//     /// An i2c bus error.
//     I2c(i2c::Error),
// }
