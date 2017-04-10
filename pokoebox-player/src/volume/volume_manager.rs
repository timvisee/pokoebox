use super::volume_control::VolumeControl;

/// Volume manager, which manages all volume controls, dials and so on.
pub struct VolumeManager {
    // TODO: Remove this allow statement after implementing
    #[allow(dead_code)]
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