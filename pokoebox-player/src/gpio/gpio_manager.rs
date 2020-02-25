#![cfg(feature = "old-rpi")]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::cupi::CuPi;

use super::pin::Pin;
use super::pin_accessor::PinAccessor;
use super::pin_config::PinConfig;
use super::pin_token::PinToken;
use error::Error;
use result::Result;

use std::thread;
use std::time::Duration;

/// Number of nanoseconds per second.
const NANOS_PER_SEC: u32 = 1_000_000_000;

/// Interval of the pin polling in seconds.
const THREAD_POLLER_INTERVAL_SEC: u64 = 0;

/// Interval of the pin polling in nanoseconds.
const THREAD_POLLER_INTERVAL_NANO: u32 = NANOS_PER_SEC / 10;

/// GPIO manager.
pub struct GpioManager {
    /// CuPi instance, to create new pin instances.
    cupi: CuPi,

    /// List of pins that are instantiated.
    pins: Arc<Mutex<HashMap<PinToken, Pin>>>,

    /// Token index, used to create an unique auto incrementing token value.
    token_index: Arc<Mutex<usize>>,
}

impl GpioManager {
    /// Constructor.
    pub fn new() -> Result<Self> {
        debug!("Initializing GPIO manager...");

        // Initialize CuPi
        let cupi = CuPi::new();
        if cupi.is_err() {
            return Err(Error::new("Failed to initialize CuPi for GPIO."));
        }

        // Construct and return
        let manager = Ok(GpioManager {
            cupi: cupi.unwrap(),
            pins: Arc::new(Mutex::new(HashMap::new())),
            token_index: Arc::new(Mutex::new(0)),
        });

        debug!("Successfully initialized GPIO manager.");

        manager
    }

    /// Get the CuPi instance.
    pub fn cupi(&self) -> &CuPi {
        &self.cupi
    }

    /// Create a pin accessor instance, that provides accessibility to the pins that are managed
    /// by the GPIO manager.
    ///
    /// This method creates a lock on the list of managed pins, to ensure concurrency safety.
    /// The lock is automatically released when the pin accessor is dropped.
    ///
    /// If an existing lock is active, the method blocks until a lock can be successfully acquired.
    pub fn pin_accessor<'a>(&'a self) -> PinAccessor<'a> {
        PinAccessor::from(&self.pins)
    }

    /// Create a new pin with the given configuration.
    ///
    /// # Errors
    ///
    /// An error is returned when pin creation failed. This might be because the internal pin
    /// creation resulted into an error.
    pub fn create_pin(&mut self, config: PinConfig) -> Result<PinToken> {
        // Create a new pin instance, get it's reference and return it's token
        Pin::from(self, config)
    }

    /// Generate a new unique pin token, that can be used to identify a new pin.
    pub fn generate_pin_token(&mut self) -> PinToken {
        // Create a lock on the token index
        let mut index = self.token_index.lock().unwrap();

        // Generate a new token
        let token = PinToken::new(*index);

        // Increase the index by one for followup tokens
        *index += 1;

        token
    }

    /// Start the polling thread.
    /// The polling thread monitors the signal of each pin, and handles the appropriate triggers
    /// when the signal of a pin changes.
    pub fn start_poll_thread(&mut self) {
        // Clone the pins atomic pointer
        let pins = self.pins.clone();

        // Start the polling thread
        thread::spawn(move || {
            debug!("Started GPIO manager polling thread");

            loop {
                // Start a scope, as the GPIO manager accessor lock may not be held when sleeping
                {
                    // Get an pin accessor lock
                    let mut accessor = PinAccessor::from(&pins);

                    // Show a status message
                    trace!("Polling GPIO pins...");

                    // Loop through the available pins to poll them
                    for pin in accessor.pins_mut() {
                        // Show a status message
                        trace!("Iterating over pin for polling... (token: {})", pin.token());

                        // Poll the pin
                        pin.poll();
                    }
                }

                // Sleep the thread until the next polling iteration
                // TODO: Dynamically determine what time to wait for here.
                thread::sleep(Duration::new(
                    THREAD_POLLER_INTERVAL_SEC,
                    THREAD_POLLER_INTERVAL_NANO,
                ));
            }

            //debug!("Stopped GPIO manager polling thread");
        });
    }
}

unsafe impl Send for GpioManager {}

unsafe impl Sync for GpioManager {}
