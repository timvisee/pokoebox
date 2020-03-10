use mpris::{DBusError, Player as MprisPlayer};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PlayerHandle(String);

impl PlayerHandle {
    pub fn from(mpris_player: &MprisPlayer) -> Self {
        Self(mpris_player.unique_name().into())
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    /// Player handle.
    pub handle: PlayerHandle,

    /// Player name.
    pub name: String,

    /// Player capabilities.
    pub capabilities: PlayerCapabilities,
}

impl Player {
    /// Construct player from MPRIS player.
    pub fn from(mpris_player: &MprisPlayer) -> Result<Self, DBusError> {
        Ok(Self {
            handle: PlayerHandle::from(mpris_player),
            name: mpris_player.identity().into(),
            capabilities: PlayerCapabilities::from(mpris_player)?,
        })
    }
}

/// List of player capabilities.
#[derive(Debug, Clone)]
pub struct PlayerCapabilities {
    pub can_play: bool,
    pub can_pause: bool,
    pub can_next: bool,
    pub can_previous: bool,
    pub can_control: bool,
}

impl PlayerCapabilities {
    /// Construct player capabilities list from MPRIS player.
    fn from(mpris_player: &MprisPlayer) -> Result<Self, DBusError> {
        Ok(Self {
            can_play: mpris_player.can_play()?,
            can_pause: mpris_player.can_pause()?,
            can_next: mpris_player.can_go_next()?,
            can_previous: mpris_player.can_go_previous()?,
            can_control: mpris_player.can_control()?,
        })
    }
}
