#![cfg(feature = "old-rpi")]

use super::event_handler::EventHandler;
use super::trigger_edge::TriggerEdge;
use result::Result;

pub struct EventManager {
    /// List of registered events.
    events: Vec<Box<EventHandler>>,
}

impl EventManager {
    /// Constructor.
    pub fn new() -> Self {
        EventManager { events: Vec::new() }
    }

    /// Register a new event handler to this manager.
    // TODO: Use tokens to identify event handlers?
    pub fn register(&mut self, event_handler: Box<EventHandler>) {
        self.events.push(event_handler);
    }

    /// Fire an event for the given trigger edge.
    /// This invokes all events that cover the given trigger edge.
    /// Returns true if the event was consumed, false if not.
    ///
    /// # Errors
    ///
    /// An error is returned on error when invoking an event.
    pub fn fire(&self, trigger_edge: TriggerEdge) -> Result<bool> {
        // Loop through the events, and invoke them
        for event in &self.events {
            // Break if the trigger edge isn't covered by this event
            if !event.trigger_edge().covers(trigger_edge) {
                continue;
            }

            // Invoke the event, and return true if it's consumed
            if event.invoke()? {
                return Ok(true);
            }
        }

        // The event wasn't consumed, return false
        Ok(false)
    }

    /// Determine the trigger edge type to globally listen for, based on all registered events in
    /// this manager.
    ///
    /// None is returned if there's no trigger edge mode to trigger at, this is when there are no
    /// registered events in this manager.
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
