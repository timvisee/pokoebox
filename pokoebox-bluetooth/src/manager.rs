use std::error::Error;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

use bluez::interface::controller::ControllerSetting;
use bluez::interface::event::Event as BlueZEvent;
use bluez::Address;
use futures::executor::block_on;
use pokoebox_common::pipe::Pipe;
use tokio_executor::park::ParkThread;
use tokio_timer::timer::Timer;

use super::device::DeviceList;
use super::driver::{Driver, DriverCmd};
use crate::util;

/// Bluetooth manager.
///
/// Spawns a background worker thread to manage a bluetooth controller.
pub struct Manager {
    pub events: Pipe<Event>,
    cmds: Pipe<DriverCmd>,
}

impl Manager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let (pipe_event, pipe_cmd) = Self::spawn_worker();
        let manager = Self {
            events: pipe_event,
            cmds: pipe_cmd,
        };

        // Poll and emit bluetooth driver state
        if let Err(err) = manager.emit_state() {
            error!("Failed to emit bluetooth driver state: {:?}", err);
        }

        Ok(manager)
    }

    /// Spawn single worker thread with bluetooth controller
    fn spawn_worker() -> (Pipe<Event>, Pipe<DriverCmd>) {
        // Channel for bluetooth events, driver commands
        let event_pipe = Pipe::default();
        let cmd_pipe = Pipe::default();

        let closure_event_pipe = event_pipe.clone();
        let closure_cmd_pipe = cmd_pipe.clone();
        thread::spawn(move || {
            // TODO: propagate error
            Self::process_loop(closure_event_pipe, closure_cmd_pipe)
                .expect("Bluetooth controller error");
        });

        (event_pipe, cmd_pipe)
    }

    fn process_loop(
        pipe_event: Pipe<Event>,
        pipe_cmd: Pipe<DriverCmd>,
    ) -> Result<(), Box<dyn Error>> {
        // Set up bluetooth controller
        let mut driver = Driver::new()?;
        let mut devices = DeviceList::default();

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

        // Allocate command listener
        let cmd_rx = pipe_cmd.listen();

        loop {
            // Process bluetooth events with timeout
            // TODO: increase timeout if no events/commands for a while
            let response = block_on(timer.timeout(driver.client.process(), Duration::from_secs(1)));
            if let Ok(response) = response {
                // TODO: propagate error
                process_bluetooth_event(
                    response.expect("failed to process bluetooth"),
                    &mut devices,
                    &pipe_event,
                );
            }

            // Process commands
            process_commands(&cmd_rx, &pipe_event, &mut driver, &mut devices);

            // TODO: break if bluetooth manager was dropped
        }
    }

    /// Poll the state from the bluetooth driver, and emit events for it.
    pub fn emit_state(&self) -> Result<(), ()> {
        // TODO: propagate errors?
        self.cmds
            .send(DriverCmd::EmitState)
            .map(|_| ())
            .map_err(|_| ())
    }

    /// Set discoverability of bluetooth controller.
    ///
    /// Discoverability is enabled for a limited time and is automatically disabled after a while,
    /// see `BT_DISCOVER_TIMEOUT`.
    pub fn set_discoverable(&self, discoverable: bool) -> Result<(), ()> {
        // TODO: propagate errors?
        self.cmds
            .send(DriverCmd::Discoverable(discoverable))
            .map(|_| ())
            .map_err(|_| ())
    }
}

/// Bluetooth driver event.
///
/// These events describe the current state, and may not necessarily be a state change.
#[derive(Clone)]
pub enum Event {
    Devices(DeviceList),
    DeviceConnected(Address, DeviceList),
    DeviceDisconnected(Address, DeviceList),
    Discoverable(bool),
    Power(bool),
}

#[inline]
fn process_bluetooth_event(
    response: bluez::interface::response::Response,
    devices: &mut DeviceList,
    pipe_event: &Pipe<Event>,
) {
    // TODO: remove this debug print
    eprintln!(">>> EVENT: {:?}", &response.event);

    // Parse bluetooth events, send over channel
    let mut events = vec![];
    match &response.event {
        BlueZEvent::NewSettings { settings, .. } => {
            // TODO: only invoke if this specific setting changed
            events.push(Event::Power(settings.contains(ControllerSetting::Powered)));
            events.push(Event::Discoverable(
                settings.contains(ControllerSetting::Discoverable),
            ));
        }
        BlueZEvent::DeviceConnected {
            address,
            address_type,
            eir_data,
            ..
        } => {
            // Add bluetooth device as trusted
            util::trust_device(*address);

            // Update device list
            devices.process_device_connected(
                *address,
                *address_type,
                util::parse_device_name(eir_data),
            );

            events.push(Event::DeviceConnected(*address, devices.clone()));
        }
        BlueZEvent::DeviceDisconnected { address, .. } => {
            // Update device list
            devices.process_device_disconnected(*address);

            events.push(Event::DeviceDisconnected(*address, devices.clone()));
        }
        _ => {}
    };
    for event in events {
        pipe_event.send(event).unwrap();
    }
}

#[inline]
fn process_commands(
    cmd_rx: &Receiver<DriverCmd>,
    pipe_event: &Pipe<Event>,
    driver: &mut Driver,
    devices: &mut DeviceList,
) {
    while let Ok(cmd) = cmd_rx.try_recv() {
        match cmd {
            DriverCmd::Discoverable(discoverable) => {
                driver
                    .set_discoverable(discoverable)
                    .expect("failed to make bluetooth device discoverable");
                let _ = pipe_event.send(Event::Discoverable(discoverable));
            }
            DriverCmd::EmitState => {
                // Get state, update device list
                let (info, connections) = driver
                    .get_state()
                    .expect("failed to make bluetooth device discoverable");
                for (address, address_type) in connections {
                    devices.process_device_connected(address, address_type, None);
                }

                // Emit events
                let _ = pipe_event.send(Event::Power(
                    info.current_settings.contains(ControllerSetting::Powered),
                ));
                let _ = pipe_event.send(Event::Discoverable(
                    info.current_settings
                        .contains(ControllerSetting::Discoverable),
                ));
                let _ = pipe_event.send(Event::Devices(devices.clone()));
            }
        }
    }
}
