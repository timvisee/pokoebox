#[cfg(feature = "rpi")]
extern crate cupi;

#[cfg(feature = "rpi")]
pub mod gpio_button;
#[cfg(feature = "rpi")]
pub mod gpio_perif_inputs;
#[cfg(feature = "rpi")]
pub mod gpio_perif_outputs;
pub mod perif;
pub mod perif_manager;
