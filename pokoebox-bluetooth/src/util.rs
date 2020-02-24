use std::process::Command;

use bluez::Address;

use crate::eir;

/// Mark a bluetooth device as trusted through the `bluetoothctl` command-line.
///
/// TODO: find way to do this through bluez API.
pub fn trust_device(address: Address) {
    // Get hex address for device
    let address_hex = address_hex(address);

    info!("Trusting bluetooth device: {}", address_hex);
    // Add bluetooth device as trusted
    match Command::new("bluetoothctl")
        .arg("trust")
        .arg(&address_hex)
        .output()
    {
        Ok(output) if !output.status.success() => {
            error!(
                "Failed to add bluetooth device as trusted, command had non-zero exit code ({}):\nstdout: {}\nstderr: {}",
                output.status.code().unwrap_or(-1),
                String::from_utf8(output.stdout).unwrap_or_else(|_| "?".into()),
                String::from_utf8(output.stderr).unwrap_or_else(|_| "?".into()),
            );
        }
        Err(err) => {
            error!("Failed to add bluetooth device as trusted: {:?}", err);
        }
        _ => {}
    }
}

/// Convert BlueZ address into hexadecimal representation with `:` separator.
pub fn address_hex(address: Address) -> String {
    let hex_address: [u8; 6] = address.into();
    hex_address
        .iter()
        .rev()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join(":")
}

/// Parse device name from EIR data.
pub fn parse_device_name(eir_data: &[u8]) -> Option<String> {
    let name = eir::parse(eir_data).and_then(|data| data.name);
    if let Some(ref name) = name {
        info!("Found device name: {}", name);
    }
    name
}
