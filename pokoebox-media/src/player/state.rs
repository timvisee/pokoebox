use super::Metadata;

// TODO: describe this is not up-to-date, and uses snapshots
#[derive(Debug, Clone)]
pub struct State {
    /// Whether this is playing.
    playing: bool,

    /// Source metadata.
    metadata: Metadata,
}

impl State {
    /// Take a snapshot of this state. Same as `clone()`.
    pub fn snapshot(&self) -> Self {
        self.clone()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            playing: false,
            metadata: Metadata::default(),
        }
    }
}
