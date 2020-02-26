use rppal::system::DeviceInfo;

/// Check whether the current system is a Raspberry Pi.
pub fn is_pi() -> Result<bool, rppal::system::Error> {
    match DeviceInfo::new() {
        Ok(_) => Ok(true),
        Err(rppal::system::Error::UnknownModel) => Ok(false),
        #[allow(unreachable_patterns)]
        Err(err) => Err(err),
    }
}
