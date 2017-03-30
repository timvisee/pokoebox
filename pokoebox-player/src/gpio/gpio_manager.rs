#![cfg(feature = "rpi")]

use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;

use super::crossbeam;
use super::crossbeam::ScopedJoinHandle;
use super::cupi::CuPi;

use error::Error;
use result::Result;
use super::pin::Pin;
use super::pin_accessor::PinAccessor;
use super::pin_config::PinConfig;
use super::pin_token::PinToken;

use std::thread;
use std::time::Duration;

/// GPIO manager.
pub struct GpioManager {
    /// CuPi instance, to create new pin instances.
    cupi: CuPi,

    /// List of pins that are instantiated.
    pins: Mutex<HashMap<PinToken, Pin>>,

    /// Token index, used to create an unique auto incrementing token value.
    token_index: AtomicUsize,

    thread: Option<ScopedJoinHandle<()>>,
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
            pins: Mutex::new(HashMap::new()),
            token_index: AtomicUsize::new(0),
            thread: None,
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
        PinAccessor::new(self.pins.lock().unwrap())
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
        // Generate a new token
        let token = PinToken::new(self.token_index.load(Ordering::Relaxed));

        // Increase the index by one for followup tokens
        self.token_index.fetch_add(1usize, Ordering::Relaxed);

        token
    }

    /// Start the polling thread.
    /// The polling thread monitors the signal of each pin, and handles the appropriate triggers
    /// when the signal of a pin changes.
    pub fn start_poll_thread(&mut self) {
        // Define whether the thread should be active
        let active = Mutex::new(true);

        // Start a scoped thread
        self.thread = Some(crossbeam::scope(|scope| {
            // Define what happens when the thread falls out of scope
            scope.defer(|| {
                // Show a status message, make the thread inactive on it's next iteration
                info!("Stopping GPIO manager polling thread...");
                *active.lock().unwrap() = false;
            });

            // Define the thread logic
            scope.spawn(|| {
                info!("Started GPIO manager polling thread");

                loop {
                    // Start a scope, as the GPIO manager accessor lock may not be held when sleeping
                    {
                        // Get an pin accessor lock
                        let mut accessor = self.pin_accessor();

                        // Show a status message
                        // TODO: Set the logging level for this message to trace
                        info!("# Polling...");

                        // Loop through the available pins to poll them
                        for pin in accessor.pins_mut() {
                            // Show a status message
                            // TODO: Set the logging level for this message to trace
                            info!("# Iterating over pin for polling... (token: {})", pin.token());
                        }
                    }

                    // Sleep the thread until the next polling iteration
                    // TODO: Dynamically determine what time to wait for here.
                    thread::sleep(Duration::new(1, 0));

                    // Break the thread if it shouldn't be active anymore
                    if !*active.lock().unwrap() {
                        break;
                    }
                }

                info!("Stopped GPIO manager polling thread");
            })
        }));
    }
}

unsafe impl Send for GpioManager {}

unsafe impl Sync for GpioManager {}
