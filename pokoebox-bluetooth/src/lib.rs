use std::error::Error;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use bluez::client::{BlueZClient, DiscoverableMode, IoCapability};
use bluez::interface::controller::{Controller, ControllerSetting};
use bluez::interface::event::Event as BlueZEvent;
use bluez::result::Error as BlueZError;
use futures_executor::block_on;

/// Publicly visible bluetooth controller name.
const BT_NAME: &str = "PokoeBox";

/// Publicly visible bluetooth controller name, short variant, with at most 10 bytes.
const BT_NAME_SHORT: Option<&str> = Some(BT_NAME);

/// Bluetooth discovery mode timeout in seconds.
const BT_DISCOVER_TIMEOUT: u16 = 60;

/// Drives a bluetooth controller for PokoeBox audio connectivity.
///
/// On creation this selects a capable bluetooth controller, and prepares it for audio
/// connectivity.
// TODO: attempt to power off controller on drop?
pub struct Driver<'a> {
    client: BlueZClient<'a>,
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

        let mut driver = Self { client, controller };

        // TODO: do not make discoverable on init?
        driver
            .set_discoverable(true)
            .expect("failed to make discoverable");

        Ok(driver)
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
        block_on(client.set_local_name(controller, BT_NAME, BT_NAME_SHORT))?;
        block_on(client.set_io_capability(controller, IoCapability::NoInputNoOutput))?;
        block_on(client.set_connectable(controller, true))?;

        Ok(())
    }

    /// Set discoverability of bluetooth controller.
    ///
    /// Discoverability is enabled for a limited time and is automatically disabled after a while,
    /// see `BT_DISCOVER_TIMEOUT`.
    pub fn set_discoverable(&mut self, discoverable: bool) -> Result<(), BlueZError> {
        let mode = if discoverable {
            DiscoverableMode::General
        } else {
            DiscoverableMode::None
        };

        block_on(self.client.set_discoverable(
            self.controller.unwrap(),
            mode,
            Some(BT_DISCOVER_TIMEOUT),
        ))
        .map(|_| ())
    }
}

/// Bluetooth manager.
///
/// Spawns a background worker thread to manage a bluetooth controller.
pub struct Manager {
    // TODO: create multi receiver/listener?
    pub events: Receiver<Event>,
    cmds: Sender<DriverCmd>,
}

impl Manager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let (event_rx, cmd_tx) = Self::spawn_worker();
        Ok(Self {
            events: event_rx,
            cmds: cmd_tx,
        })
    }

    /// Spawn single worker thread with bluetooth controller
    fn spawn_worker() -> (Receiver<Event>, Sender<DriverCmd>) {
        // Channel for bluetooth events, driver commands
        let (event_tx, event_rx) = mpsc::channel();
        let (cmd_tx, cmd_rx) = mpsc::channel();

        thread::spawn(|| {
            // TODO: propagate error
            Self::process_loop(event_tx, cmd_rx).expect("Bluetooth controller error");
        });

        (event_rx, cmd_tx)
    }

    fn process_loop(
        event_tx: Sender<Event>,
        cmd_rx: Receiver<DriverCmd>,
    ) -> Result<(), Box<dyn Error>> {
        // Set up bluetooth controller
        let mut driver = Driver::new()?;

        // TODO: stop if no controller is found
        // // We must have a controller selected
        // if data.controller.is_none() {
        //     return Ok(());
        // }

        loop {
            // Blocking until event is received
            // TODO: add timeout (to kill thread without bluetooth event on stop)
            let response = block_on(driver.client.process())?;

            // TODO: remove this debug print
            eprintln!(">>> EVENT: {:?}", &response.event);

            // Parse bluetooth events, send over channel
            let event = match &response.event {
                BlueZEvent::NewSettings { settings, .. } => {
                    // TODO: only invoke if this specific setting changed
                    Some(Event::Power(settings.contains(ControllerSetting::Powered)))
                }
                BlueZEvent::Discovering { discovering, .. } => {
                    Some(Event::Discovering(*discovering))
                }
                BlueZEvent::DeviceConnected { .. } => Some(Event::DeviceConnected),
                BlueZEvent::DeviceDisconnected { .. } => Some(Event::DeviceDisconnected),
                _ => None,
            };
            if let Some(event) = event {
                event_tx.send(event).unwrap();
            }

            // Process commands
            while let Ok(cmd) = cmd_rx.recv_timeout(Duration::from_millis(50)) {
                match cmd {
                    DriverCmd::Discoverable(discoverable) => driver
                        .set_discoverable(discoverable)
                        .expect("failed to make bluetooth device discoverable"),
                }
            }

            // TODO: break if bluetooth manager was dropped
        }
    }

    /// Set discoverability of bluetooth controller.
    ///
    /// Discoverability is enabled for a limited time and is automatically disabled after a while,
    /// see `BT_DISCOVER_TIMEOUT`.
    pub fn set_discoverable(&self, discoverable: bool) -> Result<(), ()> {
        // TODO: propagate errors?
        self.cmds
            .send(DriverCmd::Discoverable(discoverable))
            .map_err(|_| ())
    }
}

#[derive(Clone, Eq, PartialEq)]
enum DriverCmd {
    Discoverable(bool),
}

/// Bluetooth driver event.
///
/// These events describe the current state, and may not necessarily be a state change.
#[derive(Clone, Eq, PartialEq)]
pub enum Event {
    Power(bool),
    Discovering(bool),
    DeviceConnected,
    DeviceDisconnected,
}
