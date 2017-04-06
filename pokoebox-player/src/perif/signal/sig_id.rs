/// An unique ID for an input or output signal of a peripheral.
#[derive(Eq, Clone, Debug)]
pub struct SigId {
    id: &'static str
}

impl SigId {
    /// Construct a new ID.
    pub fn new(id: &'static str) -> Self {
        SigId {
            id,
        }
    }

    /// Get the ID as a string.
    pub fn id(&self) -> &'static str {
        &self.id
    }
}

impl PartialEq for SigId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}