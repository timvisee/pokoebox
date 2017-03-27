use super::sig_generic::SigGeneric;
use super::sig_in::SigIn;

/// A trait for an input signal that is generic,
/// and doesn't require any additional parameters such as a GPIO manager reference.
pub trait SigInGeneric: SigIn + SigGeneric {}