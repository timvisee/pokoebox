#[cfg(feature = "old-rpi")]
pub mod perif_gpio_button;
#[cfg(feature = "old-rpi")]
pub mod perif_gpio_light;
pub mod perif_manager;
pub mod perif_type;
pub mod signal;
pub mod traits;
