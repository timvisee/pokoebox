use error::Error;
use super::with_sig::WithSig;
use super::super::signal::sig_id::SigId;
use super::super::signal::traits::sig_in::SigIn;

/// Defines that the peripheral has input signals.
/// This may be combined with `WithOutputs`.
pub trait WithInputs: WithSig {

    /// Get a vector of inputs this peripheral provides.
    fn inputs(&self) -> &Vec<Box<SigIn>>;

    /// Get the input signal with the given ID.
    fn input(&self, id: SigId) -> Result<&SigIn, Error> {
        // Loop through the inputs
        for input in self.inputs().as_slice() {
            if input.id() == &id {
                return Ok(input.as_ref());
            }
        }

        Err(Error::new("Failed to find a requested input signal"))
    }
}