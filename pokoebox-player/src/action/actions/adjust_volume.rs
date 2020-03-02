use std::sync::Arc;

use pokoebox_audio::volume::Cmd as VolumeCmd;

use crate::action::prelude::*;
use crate::app::Core;
use crate::result::Result;

/// Name of this action.
pub const ACTION_NAME: &str = "Adjust volume";

/// Defualt step size.
const STEP_SIZE: i64 = 3;

/// Go to page action.
pub struct AdjustVolume(i64);

impl AdjustVolume {
    pub fn new(amount: i64) -> Self {
        Self(amount)
    }

    pub fn up() -> Self {
        Self::new(STEP_SIZE)
    }

    pub fn down() -> Self {
        Self::new(-STEP_SIZE)
    }
}

impl Action for AdjustVolume {
    fn name(&self) -> &'static str {
        ACTION_NAME
    }

    fn invoke(&self, core: Arc<Core>) -> Result<bool> {
        // Get master control
        let control = core.volume.get_master_control().0.clone();

        // Adjust volume
        // TODO: propagate errors here!
        core.volume
            .send_cmd(VolumeCmd::AdjustVolume(control, self.0))
            .expect("Failed to adjust volume");

        Ok(true)
    }
}
