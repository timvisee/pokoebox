use std::{mem, pin::Pin, ptr, time::Duration};

/// A tracked MPRIS player.
///
/// This creates a progress tracker along with the MPRIS player, while keeping the MPRIS player
/// intact. This allows tracking player progres changes, while keeping control over the actual
/// player.
pub(crate) struct TrackedPlayer {
    /// MPRIS player that is tracked.
    pub player: Pin<Box<mpris::Player<'static>>>,

    /// Player progress tracker.
    tracker: *mut mpris::ProgressTracker<'static>,
}

impl TrackedPlayer {
    /// Construct new tracked player, from given MPRIS player.
    pub fn new(player: mpris::Player<'static>) -> Self {
        // Construct tracked player, without the tracker
        let mut tracked = Self {
            player: Box::pin(player),
            tracker: ptr::null_mut(),
        };

        // Allocate tracker for MPRIS player
        match tracked.player.track_progress(0) {
            Ok(tracker) => {
                // Move tracker onto heap, leak into raw pointer so we can self reference
                let ptr = Box::into_raw(Box::new(tracker)) as *mut usize;
                tracked.tracker = ptr as *mut mpris::ProgressTracker<'static>;
            }
            Err(err) => {
                error!(
                    "Failed to set up progress tracker for MPRIS player: {:?}",
                    err
                );
            }
        }

        tracked
    }

    /// Get mutable reference to progress tracker.
    pub fn tracker(&mut self) -> Option<&mut mpris::ProgressTracker<'static>> {
        unsafe { self.tracker.as_mut() }
    }

    /// Do a progress tracker tick.
    ///
    /// Returns `None` if progress tracker is unassigned.
    pub fn tick(&mut self) -> Option<mpris::ProgressTick> {
        // Get tracker, tick for 10ms to process new events
        // TODO: replace with something that doesn't need to block for 10ms
        self.tracker()
            .map(|tracker| tracker.tick_for(Duration::from_millis(10)))
    }
}

impl Drop for TrackedPlayer {
    fn drop(&mut self) {
        // Explicitly drop tracker we leaked
        mem::drop(unsafe { Box::from_raw(self.tracker) });
    }
}
