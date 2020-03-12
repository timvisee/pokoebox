use super::Metadata;

// TODO: describe this is not up-to-date, and uses snapshots
#[derive(Debug, Clone)]
pub struct SourceState {
    /// Whether this is playing.
    playing: bool,

    /// Source metadata.
    metadata: Metadata,
}

impl SourceState {
    /// Take a snapshot of this state. Same as `clone()`.
    pub fn snapshot(&self) -> Self {
        self.clone()
    }
}

impl Default for SourceState {
    fn default() -> Self {
        Self {
            playing: false,
            metadata: Metadata::default(),
        }
    }
}
