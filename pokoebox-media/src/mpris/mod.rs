mod client;
mod manager;
mod player;
mod util;

// Re-export
pub use manager::Manager;

use player::{Player, PlayerHandle};

#[derive(Debug, Clone)]
pub enum Event {
    AddPlayer(PlayerHandle, Player),
    RemovePlayer(PlayerHandle),
    Players(Vec<Player>),
}

#[derive(Debug, Clone)]
pub enum Cmd {
    /// Update MPRIS player list.
    FindPlayers,

    /// Play on current player.
    Play,

    /// Pause on current player.
    Pause,

    /// Play/pause on current player.
    PlayPause,

    /// Next on current player.
    Next,

    /// Previous on current player.
    Previous,
}
