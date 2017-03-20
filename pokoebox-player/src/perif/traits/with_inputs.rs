use super::super::signal::traits::sig_in::SigIn;
use super::with_io::WithIo;

/// Defines that the peripheral has input signals.
/// This may be combined with `WithOutputs`.
pub trait WithInputs: WithIo {

    /// Get a vector of inputs this peripheral provides.
    fn inputs(&self) -> &Vec<Box<SigIn>>;
}