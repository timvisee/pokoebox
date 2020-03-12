use std::sync::Mutex;

use pokoebox_common::pipe::Pipe;

use super::source::{Event as SourceEvent, Sources};

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
        player.sources.lock()
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

/// Player events.
#[derive(Debug, Clone)]
pub enum Event {
    // TODO: source change
    Source(SourceEvent),
}
