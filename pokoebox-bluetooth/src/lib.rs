#[macro_use]
extern crate log;

use std::error::Error;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use bluez::client::{BlueZClient, DiscoverableMode, IoCapability};
use bluez::interface::controller::{Controller, ControllerInfo, ControllerSetting};
use bluez::interface::event::Event as BlueZEvent;
use bluez::result::Error as BlueZError;
use futures::executor::block_on;
use tokio_executor::park::ParkThread;
use tokio_timer::timer::Timer;

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
        block_on(client.set_local_name(controller, BT_NAME, BT_NAME_SHORT))?;
        block_on(client.set_io_capability(controller, IoCapability::NoInputNoOutput))?;
        block_on(client.set_connectable(controller, true))?;

        Ok(())
    }

    /// Get bluetooth controller state.
    pub fn get_state(&mut self) -> Result<(ControllerInfo, Vec<String>), BlueZError> {
        let controller = self.controller.unwrap();
        let info = block_on(self.client.get_controller_info(controller))?;
        let connections = block_on(self.client.get_connections(controller))?
            .into_iter()
            .map(|(a, _)| a.to_string())
            .collect();
        Ok((info, connections))
    }

    /// Set discoverability of bluetooth controller.
    ///
    /// Discoverability is enabled for a limited time and is automatically disabled after a while,
    /// see `BT_DISCOVER_TIMEOUT`.
    pub fn set_discoverable(&mut self, discoverable: bool) -> Result<(), BlueZError> {
        let controller = self.controller.unwrap();
        block_on(self.client.set_connectable(controller, true))?;
        block_on(self.client.set_discoverable(
            controller,
            if discoverable {
                DiscoverableMode::General
            } else {
                DiscoverableMode::None
            },
            if discoverable {
                Some(BT_DISCOVER_TIMEOUT)
            } else {
                None
            },
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
        let manager = Self {
            events: event_rx,
            cmds: cmd_tx,
        };

        // Poll and emit bluetooth driver state
        if let Err(err) = manager.emit_state() {
            error!("Failed to emit bluetooth driver state: {:?}", err);
        }

        Ok(manager)
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

        // Create timer thread, get handle
        let (_worker_guard, worker_token): (_, Receiver<()>) = mpsc::channel();
        let timer = {
            let (timer_tx, timer_rx) = mpsc::channel();
            thread::spawn(move || {
                // Set-up timer, pass to parent thread
                let park = ParkThread::new();
                let mut timer = Timer::new(park);
                timer_tx
                    .send(timer.handle())
                    .expect("failed to provide timer handle");

                // Keep turning timer until parent thread dies
                while let Err(mpsc::TryRecvError::Empty) = worker_token.try_recv() {
                    if let Err(err) = timer.turn(None) {
                        error!("Failed to drive bluetooth manager timer turn: {:?}", err);
                    }
                }
            });
            timer_rx.recv().expect("failed to set-up timer thread")
        };

        loop {
            // Process bluetooth events with timeout
            // TODO: increase timeout if no events/commands for a while
            let response = block_on(timer.timeout(driver.client.process(), Duration::from_secs(1)));
            if let Ok(response) = response {
                // TODO: propagate error
                process_bluetooth_event(response.expect("failed to process bluetooth"), &event_tx);
            }

            // Process commands
            process_commands(&cmd_rx, &event_tx, &mut driver);

            // TODO: break if bluetooth manager was dropped
        }
    }

    /// Poll the state from the bluetooth driver, and emit events for it.
    pub fn emit_state(&self) -> Result<(), ()> {
        // TODO: propagate errors?
        self.cmds.send(DriverCmd::EmitState).map_err(|_| ())
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
    EmitState,
}

/// Bluetooth driver event.
///
/// These events describe the current state, and may not necessarily be a state change.
#[derive(Clone, Eq, PartialEq)]
pub enum Event {
    Connections(Vec<String>),
    DeviceConnected,
    DeviceDisconnected,
    Discovering(bool),
    Power(bool),
}

#[inline]
fn process_bluetooth_event(
    response: bluez::interface::response::Response,
    event_tx: &Sender<Event>,
) {
    // TODO: remove this debug print
    eprintln!(">>> EVENT: {:?}", &response.event);

    // Parse bluetooth events, send over channel
    let mut events = vec![];
    match &response.event {
        BlueZEvent::NewSettings { settings, .. } => {
            // TODO: only invoke if this specific setting changed
            events.push(Event::Power(settings.contains(ControllerSetting::Powered)));
            events.push(Event::Discovering(
                settings.contains(ControllerSetting::Discoverable),
            ));
        }
        BlueZEvent::Discovering { discovering, .. } => {
            events.push(Event::Discovering(*discovering))
        }
        BlueZEvent::DeviceConnected { .. } => {
            events.push(Event::DeviceConnected);
        }
        BlueZEvent::DeviceDisconnected { .. } => {
            events.push(Event::DeviceDisconnected);
        }
        _ => {}
    };
    for event in events {
        event_tx.send(event).unwrap();
    }
}

#[inline]
fn process_commands(cmd_rx: &Receiver<DriverCmd>, events_tx: &Sender<Event>, driver: &mut Driver) {
    while let Ok(cmd) = cmd_rx.try_recv() {
        match cmd {
            DriverCmd::Discoverable(discoverable) => {
                driver
                    .set_discoverable(discoverable)
                    .expect("failed to make bluetooth device discoverable");
                let _ = events_tx.send(Event::Discovering(discoverable));
            }
            DriverCmd::EmitState => {
                let (info, connections) = driver
                    .get_state()
                    .expect("failed to make bluetooth device discoverable");
                let _ = events_tx.send(Event::Power(
                    info.current_settings.contains(ControllerSetting::Powered),
                ));
                let _ = events_tx.send(Event::Discovering(
                    info.current_settings
                        .contains(ControllerSetting::Discoverable),
                ));
                let _ = events_tx.send(Event::Connections(connections));
            }
        }
    }
}
