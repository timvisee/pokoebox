pub mod mpris;
pub mod traits;

use super::{Handle, RemoteHandle, State};

// Re-export
pub use self::mpris::MprisSource;

// Prelude common types
pub mod prelude {
    pub use super::traits::*;
}
