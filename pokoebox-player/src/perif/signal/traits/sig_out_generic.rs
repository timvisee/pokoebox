use super::sig_generic::SigGeneric;
use super::sig_out::SigOut;

/// A trait for an output signal that is generic,
/// and doesn't require any additional parameters such as a GPIO manager reference.
pub trait SigOutGeneric: SigOut + SigGeneric {}