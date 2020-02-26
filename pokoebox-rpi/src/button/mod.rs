mod adapter;
mod interface;

use std::collections::HashMap;

pub use interface::Interface;

lazy_static! {
    pub static ref BUTTONS: HashMap<Button, ButtonConfig> = {
        let mut m = HashMap::new();
        m.insert(Button::Action1, ButtonConfig::Push(27));
        m
    };
}

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
