mod nop;
mod rpi_i2c;
mod traits;

pub use nop::Adapter as NopAdapter;
pub use rpi_i2c::Adapter as RpiI2cAdapter;
pub use traits::Adapter;

use super::LedCmd;
use crate::rpi::Rpi;

/// Generic adapter error.
#[derive(Debug)]
pub enum Error {
    /// Adapter selection error.
    Select,

    /// A adapter error.
    Adapter,
}

/// Select proper adapter to use at runtime.
pub fn select_adapter(rpi: &mut Rpi) -> Result<Box<dyn Adapter>, Error> {
    // Load Raspberry Pi i2c adapter
    match crate::util::is_pi() {
        Ok(true) => {
            return Ok(Box::new(
                RpiI2cAdapter::new(rpi).map_err(|_| Error::Adapter)?,
            ));
        }
        Err(err) => error!(
            "Failed to detect if device is Raspberry Pi, not using its LED interface adapter: {}",
            err
        ),
        Ok(false) => {}
    }

    // No suitable adapter, use NOP
    info!("No suitable LED interface adapter found, falling back to NOP adapter");
    Ok(Box::new(NopAdapter::default()))
}
