use super::volume_control::VolumeControl;

/// Volume manager, which manages all volume controls, dials and so on.
pub struct VolumeManager {
    controls: Vec<Box<VolumeControl>>
}

impl VolumeManager {

    /// Construct a new controller.
    pub fn new() -> VolumeManager {
        VolumeManager {
            controls: Vec::new(),
        }
    }
}