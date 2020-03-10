use pokoebox_common::pipe::{Error as PipeError, Pipe};

use super::{client::Client, Cmd, Event};

/// MPRIS manager.
pub struct Manager {
    /// MPRIS client.
    client: Client,
}

impl Manager {
    /// Construct new manager.
    pub fn new() -> Self {
        // Create MPRIS client
        let client = Client::new();

        // Submit command to find players
        if let Err(err) = client.cmds.send(Cmd::FindPlayers) {
            error!(
                "Failed to submit command to MPRIS client to find players: {:?}",
                err
            );
        }

        Self { client }
    }

    /// Get events pipe.
    pub fn events<'a>(&'a self) -> &'a Pipe<Event> {
        &self.client.events
    }

    /// Send command to the client.
    pub fn send_cmd(&self, cmd: Cmd) -> Result<(), PipeError> {
        self.client.cmds.send(cmd).map(|_| ())
    }

    /// Find currently available MPRIS players.
    pub fn find_players(&self) -> Result<(), PipeError> {
        self.send_cmd(Cmd::FindPlayers)
    }
}
