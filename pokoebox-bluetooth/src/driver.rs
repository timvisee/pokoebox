use std::error::Error;

use bluez::client::{AddressType, BlueZClient, DiscoverableMode, IoCapability};
use bluez::interface::controller::{Controller, ControllerInfo, ControllerSetting};
use bluez::result::Error as BlueZError;
use bluez::Address;
use futures::executor::block_on;

/// Drives a bluetooth controller for PokoeBox audio connectivity.
///
/// On creation this selects a capable bluetooth controller, and prepares it for audio
/// connectivity.
// TODO: attempt to power off controller on drop?
pub struct Driver<'a> {
    pub(crate) client: BlueZClient<'a>,
    controller: Option<Controller>,
}

impl<'a> Driver<'a> {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // Build client, find controller, initialize
        let mut client = BlueZClient::new()?;
        let controller = Self::select_controller(&mut client)?;
        if let Some(controller) = controller {
            Self::init_controller(&mut client, controller)?;
        }

        Ok(Self { client, controller })
    }

    fn select_controller(client: &mut BlueZClient) -> Result<Option<Controller>, Box<dyn Error>> {
        let controllers = block_on(client.get_controller_list())?;

        // Select first controller we can power
        Ok(controllers
            .into_iter()
            .filter_map(|controller| {
                let info = block_on(client.get_controller_info(controller)).ok()?;

                if info.supported_settings.contains(ControllerSetting::Powered) {
                    Some(controller)
                } else {
                    None
                }
            })
            .next())
    }

    fn init_controller(
        client: &mut BlueZClient,
        controller: Controller,
    ) -> Result<(), Box<dyn Error>> {
        block_on(client.set_powered(controller, true))?;
        block_on(client.set_local_name(controller, crate::BT_NAME, crate::BT_NAME_SHORT))?;
        block_on(client.set_io_capability(controller, IoCapability::NoInputNoOutput))?;
        block_on(client.set_connectable(controller, true))?;

        Ok(())
    }

    /// Get bluetooth controller state.
    pub fn get_state(
        &mut self,
    ) -> Result<(ControllerInfo, Vec<(Address, AddressType)>), BlueZError> {
        let controller = self.controller.unwrap();
        let info = block_on(self.client.get_controller_info(controller))?;
        let connections = block_on(self.client.get_connections(controller))?;
        Ok((info, connections))
    }

    /// Set discoverability of bluetooth controller.
    ///
    /// Discoverability is enabled for a limited time and is automatically disabled after a while,
    /// see `BT_DISCOVER_TIMEOUT`.
    pub fn set_discoverable(&mut self, discoverable: bool) -> Result<(), BlueZError> {
        let controller = self.controller.unwrap();
        block_on(self.client.set_connectable(controller, true))?;
        block_on(self.client.set_bondable(controller, true))?;
        block_on(self.client.set_discoverable(
            controller,
            if discoverable {
                DiscoverableMode::General
            } else {
                DiscoverableMode::None
            },
            if discoverable {
                Some(crate::BT_DISCOVER_TIMEOUT)
            } else {
                None
            },
        ))
        .map(|_| ())
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) enum DriverCmd {
    Discoverable(bool),
    EmitState,
}
