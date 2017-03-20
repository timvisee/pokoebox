use super::super::sig_id::SigId;

/// An input or output signal for a peripheral.
pub trait Sig {

    /// Get the ID of this input/output.
    fn id(&self) -> &SigId;

    /// Get the display name of this input/output.
    fn name(&self) -> &'static str;
}