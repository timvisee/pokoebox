use super::super::signal::sig_id::SigId;
use super::super::signal::traits::sig_out::SigOut;
use super::with_sig::WithSig;
use error::Error;
use result::Result;

/// Defines that the peripheral has output signals.
/// This may be combined with `WithInputs`.
pub trait WithOutputs: WithSig {
    /// Create a vector and list all output signals in it for this peripheral.
    fn list_outputs(&self) -> Vec<&dyn SigOut>;

    /// Create a vector and list all output signals in it for this peripheral.
    /// Return the signal with the given `id`. An error is returned if no
    /// signal is found with the given ID.
    fn find_output(&self, id: SigId) -> Result<&dyn SigOut> {
        // Loop through the outputs
        for output in self.list_outputs().as_slice() {
            if output.id() == &id {
                return Ok(*output);
            }
        }

        Err(Error::new("Failed to find a requested output signal"))
    }
}
