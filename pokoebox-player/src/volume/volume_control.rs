pub trait VolumeControl {

    /// Get the minimum possible volume.
    fn min(&self) -> f64;

    /// Get the maximum possible volume.
    fn max(&self) -> f64;

    /// Get the current volume value.
    fn value(&self) -> f64;

    /// Set the current volume value.
    fn set_value(&mut self, volume: f64);

    /// Increase the current volume by the given `volume` amount.
    fn increase_volume(&mut self, volume: f64) {
        // Get the current volume value
        let cur = self.value();

        // Set the new value
        self.set_value(cur + volume)
    }

    /// Decrease the current volume by the given `volume` amount.
    fn decrease_volume(&mut self, volume: f64) {
        self.increase_volume(-volume)
    }

    /// Get a name for this volume control.
    fn name(&self) -> &'static str;
}