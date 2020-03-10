/// Generic source trait.
pub trait Source: Send + Sync {
    /// Check whether this source is currently playing.
    fn is_playing(&self) -> bool;

    /// Execute the given operation.
    fn do_operation(&self, op: Operation) -> bool;

    /// Check whether this source supports the given operation.
    fn has_operation(op: Operation) -> bool;
}

/// List of supported source operations.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operation {
    Play,
    Pause,
    PlayPause,
    Stop,
    Next,
    Previous,
}
