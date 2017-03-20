/// An unique ID for an input or output signal of a peripheral.
#[derive(Eq, Clone)]
pub struct IoId {
    id: &'static str
}

impl IoId {

    /// Construct a new ID.
    pub fn new(id: &'static str) -> Self {
        IoId {
            id: id
        }
    }

    /// Get the ID as a string.
    pub fn id(&self) -> &'static str {
        &self.id
    }
}

impl PartialEq for IoId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}