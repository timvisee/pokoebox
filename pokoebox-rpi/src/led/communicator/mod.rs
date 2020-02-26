mod nop;
mod rpi_i2c;
mod traits;

use rppal::system::DeviceInfo;

pub use nop::Communicator as NopCommunicator;
pub use rpi_i2c::Communicator as RpiI2cCommunicator;
pub use traits::Communicator;

use super::LedCmd;

/// Generic communicator error.
#[derive(Debug)]
pub enum Error {
    /// Communicator selection error.
    Select,

    /// A communication error.
    Commnicate,
}

/// Select proper communiator to use at runtime.
pub fn select_communicator() -> Result<Box<dyn Communicator>, Error> {
    // Load Raspberry Pi i2c communicator
    match is_pi() {
        Ok(true) => {
            return Ok(Box::new(RpiI2cCommunicator::new().map_err(|_| Error::Commnicate)?));
        },
        Err(err) => error!("Failed to detect if device is Raspberry Pi, not using its LED controller communicator: {}", err),
        Ok(false) => {}
    }

    // No suitable communicator, use NOP
    info!("No suitable LED controller communicator found, falling back to NOP communicator");
    Ok(Box::new(NopCommunicator::default()))
}

/// Check whether the current system is a Raspberry Pi.
fn is_pi() -> Result<bool, rppal::system::Error> {
    match DeviceInfo::new() {
        Ok(_) => Ok(true),
        Err(rppal::system::Error::UnknownModel) => Ok(false),
        #[allow(unreachable_patterns)]
        Err(err) => Err(err),
    }
}
