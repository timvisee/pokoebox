mod adapter;
mod interface;

pub use interface::{Error, Interface};

/// A power controller command.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cmd {
    /// Poll power state.
    Poll,
}

/// A power event.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Event {
    /// Power update.
    Power,
}
