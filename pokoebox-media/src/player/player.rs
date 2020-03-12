use std::sync::Mutex;

use pokoebox_common::pipe::Pipe;

use super::sources::prelude::*;
use super::{SourceHandle, SourceRemoteHandle, SourceState};

/// A generic player, having many sources.
pub struct Player
where
    Self: Send + Sync,
{
    /// Sources in this player.
    pub sources: Mutex<Sources>,

    /// Player events.
    pub events: Pipe<Event>,
}

impl Default for Player {
    fn default() -> Self {
        // Construct player
        let player = Player {
            sources: Default::default(),
            events: Default::default(),
        };

        // Forward source list events to player event stream
        let player_events = player.events.clone();
        player
            .sources
            .lock()
            .expect("failed to obtain lock on player sources")
            .events
            .register_callback(move |event| {
                if let Err(err) = player_events.send(Event::Source(event)) {
                    error!(
                        "Failed to forward player source manager event to player event stream: {:?}",
                        err
                    );
                }
            });

        player
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    // TODO: source change
    Source(SourceEvent),
}

#[derive(Debug, Clone)]
pub enum SourceEvent {
    /// A new source is added.
    Add(SourceHandle, SourceState),

    /// A source is removed.
    Remove(SourceHandle),

    /// Up-to-date list of all available sources and a snapshot of their state.
    States(Vec<(SourceHandle, SourceState)>),
}

/// A list of sources.
#[derive(Default)]
pub struct Sources {
    /// List of sources.
    sources: Vec<Box<dyn Source>>,

    /// Sources events.
    events: Pipe<SourceEvent>,
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
        if let Err(err) = self.events.send(SourceEvent::Add(handle, state)) {
            error!("Failed to emit player sources event: {:?}", err);
        }
        self.emit_sources();
    }

    pub fn remove(&mut self, handle: SourceHandle) -> bool {
        // Find source index
        let i = match self.sources.iter().position(|s| s.handle() == handle) {
            Some(i) => i,
            None => return false,
        };

        // Remove source
        self.sources.remove(i);

        // Emit events
        if let Err(err) = self.events.send(SourceEvent::Remove(handle)) {
            error!("Failed to emit player sources event: {:?}", err);
        }
        self.emit_sources();

        true
    }

    pub fn remove_remote_handle(&mut self, handle: &SourceRemoteHandle) -> bool {
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
    fn emit_sources(&self) {
        let sources = self
            .sources
            .iter()
            .map(|s| (s.handle(), s.state().snapshot()))
            .collect();
        if let Err(err) = self.events.send(SourceEvent::States(sources)) {
            error!("Failed to emit player sources event: {:?}", err);
        }
    }
}
