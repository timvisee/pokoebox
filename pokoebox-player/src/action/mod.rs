pub mod actions;
pub mod manager;
pub mod traits;

pub use self::manager::ActionManager;

/// Prelude the action traits
pub mod prelude {
    pub use super::traits::*;
}
