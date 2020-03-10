mod adapter;
mod interface;

pub use interface::{Error, Interface};

/// A power controller command.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cmd {
    /// Poll power state.
    // TODO: rename this
    Poll,
}

/// A power event.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Event {
    /// Power update (Ampere, Volt, Watt).
    Power(f32, f32, f32),
}
