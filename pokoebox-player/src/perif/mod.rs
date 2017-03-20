#[cfg(feature = "rpi")]
extern crate cupi;

pub mod button;
pub mod perif;
#[cfg(feature = "rpi")]
pub mod perif_gpio_button;
#[cfg(feature = "rpi")]
pub mod perif_gpio_light;
pub mod perif_manager;
pub mod perif_type;
pub mod signal;
pub mod traits;
