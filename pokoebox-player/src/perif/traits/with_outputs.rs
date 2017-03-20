use super::with_sig::WithSig;
use super::super::signal::traits::sig_out::SigOut;

/// Defines that the peripheral has output signals.
/// This may be combined with `WithInputs`.
pub trait WithOutputs: WithSig {

    /// Get a vector of inputs this peripheral provides.
    fn outputs(&self) -> &Vec<Box<SigOut>>;
}