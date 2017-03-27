use super::sig::Sig;

/// A trait for an input/output signal that is generic,
/// and doesn't require any additional parameters such as a GPIO manager reference.
pub trait SigGeneric: Sig {}