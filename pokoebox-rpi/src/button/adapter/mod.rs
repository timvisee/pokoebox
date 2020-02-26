mod nop;
mod rpi_gpio;
mod traits;

pub use nop::Adapter as NopAdapter;
pub use rpi_gpio::Adapter as RpiGpioAdapter;
pub use traits::Adapter;

use super::{ButtonConfig, Event};

/// Generic adapter error.
#[derive(Debug)]
pub enum Error {
    /// Adapter selection error.
    Select,

    /// A adapter error.
    Adapter,
}

/// Select proper adapter to use at runtime.
pub fn select_adapter() -> Result<Box<dyn Adapter>, Error> {
    // Load Raspberry Pi GPIO adapter
    match crate::util::is_pi() {
        Ok(true) => {
            return Ok(Box::new(
                RpiGpioAdapter::new().map_err(|_| Error::Adapter)?,
            ));
        }
        Err(err) => error!(
            "Failed to detect if device is Raspberry Pi, not using its button interface adapter: {}",
            err
        ),
        Ok(false) => {}
    }

    // No suitable adapter, use NOP
    info!("No suitable button interface adapter found, falling back to NOP adapter");
    Ok(Box::new(NopAdapter::default()))
}
