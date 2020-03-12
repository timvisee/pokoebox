use super::prelude::*;
use super::State;
use super::{Handle, RemoteHandle};
use crate::mpris::{Player, PlayerHandle};

pub struct MprisSource {
    /// Unique source handle.
    handle: Handle,

    /// MPRIS player handle.
    player_handle: PlayerHandle,

    /// MPRIS player.
    player: Player,

    /// The last source state snapshot.
    state: State,
}

impl MprisSource {
    pub fn from(player_handle: PlayerHandle, player: Player) -> Self {
        Self {
            handle: Handle::unique(),
            player_handle,
            player,
            state: State::default(),
        }
    }
}

/// Generic source trait.
impl Source for MprisSource {
    fn handle(&self) -> Handle {
        self.handle
    }

    fn remote_handle(&self) -> RemoteHandle {
        RemoteHandle::Mpris(self.player_handle.clone())
    }

    fn name(&self) -> &str {
        &self.player.name
    }

    fn is_playing(&self) -> bool {
        // TODO: return proper value here
        todo!()
    }

    fn do_operation(&self, _op: Operation) -> bool {
        // TODO: invoke action on MPRIS player
        todo!()
    }

    fn has_operation(&self, op: Operation) -> bool {
        match op {
            Operation::Play => self.player.capabilities.can_play,
            Operation::Pause => self.player.capabilities.can_pause,
            Operation::PlayPause => {
                self.player.capabilities.can_play && self.player.capabilities.can_pause
            }
            Operation::Stop => self.player.capabilities.can_stop,
            Operation::Next => self.player.capabilities.can_next,
            Operation::Previous => self.player.capabilities.can_previous,
        }
    }

    fn state(&self) -> &State {
        &self.state
    }
}
