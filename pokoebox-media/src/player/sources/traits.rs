use super::{Handle, RemoteHandle, State};

/// Generic source trait.
pub trait Source: Send + Sync {
    /// An unique handle to this source instance.
    fn handle(&self) -> Handle;

    /// An unique handle to the remote/external source component.
    // TODO: change this?
    fn remote_handle(&self) -> RemoteHandle;

    /// A source name, typically the device name.
    fn name(&self) -> &str;

    /// Check whether this source is currently playing.
    fn is_playing(&self) -> bool;

    /// Execute the given operation.
    fn do_operation(&self, op: Operation) -> bool;

    /// Check whether this source supports the given operation.
    fn has_operation(&self, op: Operation) -> bool;

    /// Get the source state.
    fn state(&self) -> &State;
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
