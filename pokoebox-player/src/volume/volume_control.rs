pub trait VolumeControl {

    /// Get the minimum possible volume.
    fn min(&self) -> f64;

    /// Get the maximum possible volume.
    fn max(&self) -> f64;

    /// Get the current volume value.
    fn value(&self) -> f64;

    /// Get a name for this volume control.
    fn name(&self) -> &'static str;
}