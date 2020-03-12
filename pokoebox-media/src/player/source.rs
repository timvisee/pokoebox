use pokoebox_common::pipe::Pipe;

use super::sources::prelude::*;
use super::{Handle, RemoteHandle, State};

/// A list of sources.
#[derive(Default)]
pub struct Sources {
    /// List of sources.
    sources: Vec<Box<dyn Source>>,

    /// Sources events.
    pub(crate) events: Pipe<Event>,
}

impl Sources {
    /// Get the main source if there is any.
    ///
    /// This is likely the source that is actively playing.
    pub fn main(&self) -> Option<&Box<dyn Source>> {
        self.sources.get(0)
    }

    /// List all sources currently playing.
    pub fn list_playing(&self) -> Vec<&Box<dyn Source>> {
        self.sources.iter().filter(|s| s.is_playing()).collect()
    }

    pub fn add(&mut self, source: Box<dyn Source>) {
        // TODO: remove after debugging
        info!("Adding new source: {}", source.name());
        let handle = source.handle();
        let state = source.state().clone();

        // Add new source
        self.sources.push(source);
        self.sort();

        // Emit events
        if let Err(err) = self.events.send(Event::Add(handle, state)) {
            error!("Failed to emit player sources event: {:?}", err);
        }
        self.emit_states();
    }

    pub fn remove(&mut self, handle: Handle) -> bool {
        // Find source index
        let i = match self.sources.iter().position(|s| s.handle() == handle) {
            Some(i) => i,
            None => return false,
        };

        // Remove source
        self.sources.remove(i);

        // Emit events
        if let Err(err) = self.events.send(Event::Remove(handle)) {
            error!("Failed to emit player sources event: {:?}", err);
        }
        self.emit_states();

        true
    }

    pub fn remove_remote_handle(&mut self, handle: &RemoteHandle) -> bool {
        let handle = match self.sources.iter().find(|s| &s.remote_handle() == handle) {
            Some(source) => source.handle(),
            None => return false,
        };
        self.remove(handle)
    }

    /// Internally sort list of sources.
    fn sort(&mut self) {
        self.sources.sort_unstable_by_key(|s| s.is_playing());
    }

    /// Emit an event for all current source states.
    pub fn emit_states(&self) {
        let sources = self
            .sources
            .iter()
            .map(|s| (s.handle(), s.state().snapshot()))
            .collect();
        if let Err(err) = self.events.send(Event::States(sources)) {
            error!("Failed to emit player sources event: {:?}", err);
        }
    }
}

/// Player source events.
#[derive(Debug, Clone)]
pub enum Event {
    /// A new source is added.
    Add(Handle, State),

    /// A source is removed.
    Remove(Handle),

    /// Up-to-date list of all available sources and a snapshot of their state.
    States(Vec<(Handle, State)>),
}
