use super::prelude::*;

pub struct MprisSource {}

/// Generic source trait.
impl Source for MprisSource {
    fn is_playing(&self) -> bool {
        // TODO: return proper value here
        false
    }

    fn do_operation(&self, _op: Operation) -> bool {
        // TODO: invoke action on MPRIS player
        false
    }

    fn has_operation(_op: Operation) -> bool {
        // TODO: determine if invokable on MPRIS player
        false
    }
}
