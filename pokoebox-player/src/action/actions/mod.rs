pub mod adjust_volume;
pub mod goto_page;
pub mod nop;

// Re-export actions
pub use adjust_volume::AdjustVolume;
pub use goto_page::GotoPageAction;
pub use nop::NopAction;
