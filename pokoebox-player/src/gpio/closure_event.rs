#![cfg(feature = "old-rpi")]

use super::event_handler::EventHandler;
use super::trigger_edge::TriggerEdge;
use error::Error;
use result::Result;

pub enum ClosureEvent {
    // Event with an unmutable closure
    EventFn(Box<Fn() -> Result<bool>>, TriggerEdge),

    // Event with a mutable closure
    EventFnMut(Box<FnMut() -> Result<bool>>, TriggerEdge),
}

impl ClosureEvent {
    /// Check whether this event closure is mutable or not.
    pub fn is_mut(&self) -> bool {
        match *self {
            ClosureEvent::EventFn(..) => false,
            ClosureEvent::EventFnMut(..) => true,
        }
    }
}

impl EventHandler for ClosureEvent {
    fn invoke(&self) -> Result<bool> {
        match *self {
            ClosureEvent::EventFn(ref closure, _) => closure(),
            ClosureEvent::EventFnMut(..) => Err(Error::new("This event must be invoked mutably.")),
        }
    }

    fn invoke_mut(&mut self) -> Result<bool> {
        match *self {
            ClosureEvent::EventFn(ref closure, _) => closure(),
            ClosureEvent::EventFnMut(ref mut mut_closure, _) => mut_closure(),
        }
    }

    fn trigger_edge(&self) -> TriggerEdge {
        match *self {
            ClosureEvent::EventFn(_, trigger_edge) => trigger_edge,
            ClosureEvent::EventFnMut(_, trigger_edge) => trigger_edge,
        }
    }
}
