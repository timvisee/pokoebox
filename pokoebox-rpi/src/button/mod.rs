mod adapter;
mod interface;

pub use interface::{Error, Interface};

/// List of available buttons.
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Button {
    /// Maion action button 1.
    Action1,

    /// Maion action button 2.
    Action2,

    /// Maion action button 3.
    Action3,

    /// Maion action button 4.
    Action4,

    /// Volume rotary encoder.
    Volume,

    /// Selection rotary encoder.
    Select,

    /// LED in power button.
    Power,
}

/// Button configuration.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ButtonConfig {
    /// A regular push button.
    Push(u8),

    /// A rotary encoder.
    Rotary(u8, u8),
}

/// A button event.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Event {
    /// Button is pressed.
    Press,

    /// Button is rotated up.
    Up,

    /// Button is rotated down.
    Down,
}
