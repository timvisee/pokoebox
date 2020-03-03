pub mod control;
pub mod manager;
mod mixer;
mod util;

use std::collections::HashMap;

// Re-export
pub use control::{ControlHandle, ControlProps};
pub use manager::Manager;

#[derive(Clone, Debug)]
pub enum Cmd {
    /// Reset volume to defaults.
    ResetVolume,

    /// Request list of controls.
    GetControls,

    /// Get volume of given control.
    GetVolume(ControlHandle),

    /// Set volume of given control.
    SetVolume(ControlHandle, i64),

    /// Adjust volume of given control.
    AdjustVolume(ControlHandle, i64),
}

#[derive(Clone, Debug)]
pub enum Event {
    /// List of all control handles and properties.
    Controls(HashMap<ControlHandle, ControlProps>),

    /// Current volume for control.
    Volume(ControlHandle, i64),
}
