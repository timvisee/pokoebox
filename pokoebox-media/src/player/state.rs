use super::Metadata;

// TODO: describe this is not up-to-date, and uses snapshots
#[derive(Debug, Clone)]
pub struct State {
    /// Source name.
    pub name: String,

    /// Whether this is playing.
    pub playing: bool,

    /// Source metadata.
    pub metadata: Metadata,
}

impl State {
    /// Construct a new state.
    pub fn new(name: String) -> Self {
        Self {
            name,
            playing: false,
            metadata: Metadata::default(),
        }
    }

    /// Take a snapshot of this state. Same as `clone()`.
    pub fn snapshot(&self) -> Self {
        self.clone()
    }
}
