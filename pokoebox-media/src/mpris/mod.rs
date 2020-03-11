mod client;
mod manager;
mod player;
mod tracked;
mod util;

// Re-export
pub use manager::Manager;

use player::{Player, PlayerHandle};

/// MPRIS manager events.
#[derive(Debug, Clone)]
pub enum Event {
    /// A new MPRIS player is available.
    AddPlayer(PlayerHandle, Player),

    /// A MPRIS player has been removed.
    RemovePlayer(PlayerHandle),

    /// Track info.
    // TODO: remove this temporary event
    TrackInfo(String),

    /// A list of all currently available MPRIS players.
    Players(Vec<Player>),
}

/// MPRIS manager commands.
#[derive(Debug, Clone)]
pub enum Cmd {
    /// Update MPRIS player list.
    FindPlayers,

    /// Play on current player.
    // TODO: link operation to specific player
    Play,

    /// Pause on current player.
    // TODO: link operation to specific player
    Pause,

    /// Play/pause on current player.
    // TODO: link operation to specific player
    PlayPause,

    /// Next on current player.
    // TODO: link operation to specific player
    Next,

    /// Previous on current player.
    // TODO: link operation to specific player
    Previous,
}
