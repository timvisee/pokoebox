use super::super::io_id::IoId;

/// An input or output signal for a peripheral.
pub trait Io {

    /// Get the ID of this input/output.
    fn id(&self) -> &IoId;

    /// Get the display name of this input/output.
    fn name(&self) -> &'static str;
}