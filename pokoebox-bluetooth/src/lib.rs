#[macro_use]
extern crate log;

pub mod device;
pub mod driver;
pub mod eir;
pub mod manager;
pub mod util;

/// Publicly visible bluetooth controller name.
pub const BT_NAME: &str = "PokoeBox";

/// Publicly visible bluetooth controller name, short variant, with at most 10 bytes.
pub const BT_NAME_SHORT: Option<&str> = Some(BT_NAME);

/// Bluetooth discovery mode timeout in seconds.
pub(crate) const BT_DISCOVER_TIMEOUT: u16 = 60;
