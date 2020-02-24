use std::slice::Iter;

use bluez::client::AddressType;
use bluez::Address;

use crate::util;

/// Represents a bluetooth device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub address: Address,
    pub address_type: AddressType,
    pub name: Option<String>,
    pub connected: bool,
}

impl Device {
    /// Construct new default device with given address.
    pub fn from_address(address: Address, address_type: AddressType) -> Self {
        Self {
            address,
            address_type,
            name: None,
            connected: true,
        }
    }

    /// Get address as human readable hex string separated by `:`.
    pub fn address_string(&self) -> String {
        util::address_hex(self.address)
    }
}

/// Represents a list of bluetooth devices.
#[derive(Clone)]
pub struct DeviceList {
    devices: Vec<Device>,
}

impl DeviceList {
    /// Process device connection event, to update device list.
    pub(crate) fn process_device_connected(
        &mut self,
        address: Address,
        address_type: AddressType,
        name: Option<String>,
    ) {
        match self.get_mut(address) {
            Some(device) => {
                device.address_type = address_type;
                device.connected = true;
                if let Some(name) = name {
                    device.name.replace(name);
                }
            }
            None => {
                self.devices
                    .push(Device::from_address(address, address_type));
            }
        }
    }

    /// Process device disconnect event, to update device list.
    pub(crate) fn process_device_disconnected(&mut self, address: Address) {
        if let Some(device) = self.get_mut(address) {
            device.connected = false;
        }
    }

    /// Get device by address if exists.
    pub fn get(&self, address: Address) -> Option<&Device> {
        self.devices.iter().find(|d| d.address == address)
    }

    /// Get device by address as mutable if exists.
    pub fn get_mut(&mut self, address: Address) -> Option<&mut Device> {
        self.devices.iter_mut().find(|d| d.address == address)
    }

    /// Get device iterator.
    pub fn iter(&self) -> Iter<Device> {
        self.devices.iter()
    }
}

impl Default for DeviceList {
    fn default() -> Self {
        Self {
            devices: Vec::new(),
        }
    }
}
