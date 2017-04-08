#![cfg(feature = "rpi")]

use super::event_handler::EventHandler;
use super::trigger_edge::TriggerEdge;

pub struct EventManager {
    /// List of registered events.
    events: Vec<Box<EventHandler>>,
}

impl EventManager {
    /// Constructor.
    pub fn new() -> Self {
        EventManager {
            events: Vec::new(),
        }
    }

    /// Determine the trigger edge type to globally listen for, based on all registered events in
    /// this manager.
    pub fn concatenated_trigger_edge(&self) -> Option<TriggerEdge> {
        // Return none if no event is registered
        if self.events.len() == 0 {
            return None;
        }

        // Concatenate the trigger edge, to determine the global type to use
        let mut trigger_edge: Option<TriggerEdge> = None;
        for entry in &self.events {
            match trigger_edge {
                Some(edge) => trigger_edge = Some(edge.concat(entry.trigger_edge())),
                None => trigger_edge = Some(entry.trigger_edge()),
            }
        }

        // Return the concatenated trigger edge
        trigger_edge
    }
}