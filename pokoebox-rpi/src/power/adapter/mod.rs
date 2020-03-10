mod nop;
mod rpi_i2c_ina260;
mod traits;

use pokoebox_common::pipe::Pipe;

use super::{Cmd, Event};
use crate::rpi::Rpi;

// Re-export
pub use nop::Adapter as NopAdapter;
pub use rpi_i2c_ina260::Adapter as RpiIna260Adapter;
pub use traits::Adapter;

/// Generic adapter error.
#[derive(Debug)]
pub enum Error {
    /// Adapter selection error.
    Select,

    /// A adapter error.
    Adapter,
}

/// Select proper adapter to use at runtime.
pub fn select_adapter(rpi: &mut Rpi, events: Pipe<Event>) -> Result<Box<dyn Adapter>, Error> {
    // Load Raspberry Pi GPIO adapter
    match crate::util::is_pi() {
        Ok(true) => {
            return Ok(Box::new(
                RpiIna260Adapter::new(rpi, events).map_err(|_| Error::Adapter)?,
            ));
        }
        Err(err) => error!(
            "Failed to detect if device is Raspberry Pi, not using its power interface adapter: {}",
            err
        ),
        Ok(false) => {}
    }

    // No suitable adapter, use NOP
    info!("No suitable power interface adapter found, falling back to NOP adapter");
    Ok(Box::new(NopAdapter::default()))
}
