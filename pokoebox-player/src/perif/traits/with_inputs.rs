use super::super::signal::sig_id::SigId;
use super::super::signal::traits::sig_in::SigIn;
use super::with_sig::WithSig;
use crate::error::Error;
use crate::result::Result;

/// Defines that the peripheral has input signals.
/// This may be combined with `WithOutputs`.
pub trait WithInputs: WithSig {
    /// Create a vector and list all input signals in it for this peripheral.
    fn list_inputs(&self) -> Vec<&dyn SigIn>;

    /// Create a vector and list all input signals in it for this peripheral.
    /// Return the signal with the given `id`. An error is returned if no
    /// signal is found with the given ID.
    fn find_input(&self, id: SigId) -> Result<&dyn SigIn> {
        // Loop through the inputs
        for input in self.list_inputs().as_slice() {
            if input.id() == &id {
                return Ok(*input);
            }
        }

        Err(Error::new("Failed to find a requested input signal"))
    }
}
