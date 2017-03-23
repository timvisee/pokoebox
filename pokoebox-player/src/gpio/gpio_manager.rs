#![cfg(feature = "rpi")]

extern crate mio;

use super::cupi::CuPi;
use mio::{Poll, Events, Token, Ready, PollOpt, timer};
use super::cupi::sys::Edge;

use error::Error;
use result::Result;

/// GPIO manager.
pub struct GpioManager {
    cupi: CuPi,
    poll: Poll,
}

impl GpioManager {
    /// Constructor.
    pub fn new() -> Result<Self> {
        debug!("Initializing GPIO manager...");

        // Initialize CuPi
        let cupi = CuPi::new();
        if cupi.is_err() {
            return Err(Error::new("Failed to initialize CuPi for GPIO."));
        }

        // Initialize a poll
        let poll = Poll::new();
        if poll.is_err() {
            return Err(Error::new("Failed to initialize polling object for GPIO."));
        }

        // Construct and return
        let manager = Ok(GpioManager {
            cupi: cupi.unwrap(),
            poll: poll.unwrap(),
        });

        debug!("Successfully initialized GPIO manager.");

        manager
    }

    /// Get the CuPi instance.
    pub fn cupi(&self) -> &CuPi {
        &self.cupi
    }

    /// Get the polling object.
    pub fn poll(&self) -> &Poll {
        &self.poll
    }

    /// Get a mutable polling object.
    pub fn poll_mut(&mut self) -> &mut Poll {
        &mut self.poll
    }
}