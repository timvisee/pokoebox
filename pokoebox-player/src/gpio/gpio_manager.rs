#![cfg(feature = "rpi")]

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use super::cupi::CuPi;

use error::Error;
use result::Result;
use super::pin::Pin;
use super::pin_config::PinConfig;
use super::pin_token::PinToken;

/// Number of nanoseconds per second.
const NANOS_PER_SEC: u32 = 1000000000;

/// Interval of the pin polling.
const THREAD_POLLER_INTERVAL_NANO: u32 = NANOS_PER_SEC;

/// GPIO manager.
pub struct GpioManager {
    /// CuPi instance, to create new pin instances.
    cupi: CuPi,

    /// List of pins that are instantiated.
    pins: HashMap<PinToken, Pin>,

    /// Token index, used to create an unique auto incrementing token value.
    token_index: usize
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

        // Create a new polling thread
        thread::spawn(|| {
            loop {
                println!("WORKER THREAD HERE!");

                // Sleep the worker thread
                thread::sleep(Duration::new(0, THREAD_POLLER_INTERVAL_NANO));
            }
        });

        // Construct and return
        let manager = Ok(GpioManager {
            cupi: cupi.unwrap(),
            pins: HashMap::new(),
            token_index: 0,
        });

        debug!("Successfully initialized GPIO manager.");

        manager
    }

    /// Get the CuPi instance.
    pub fn cupi(&self) -> &CuPi {
        &self.cupi
    }

    /// Create a new pin with the given configuration.
    ///
    /// # Errors
    ///
    /// An error is returned when pin creation failed. This might be because the internal pin
    /// creation resulted into an error.
    pub fn create_pin(&mut self, config: PinConfig) -> Result<(PinToken, &Pin)> {
        // Create a new pin instance, get it's reference and return it's token
        Pin::from(self, config)
    }

    /// Add the given pin to the manager.
    /// A reference to the added pin is returned.
    pub fn add_pin(&mut self, pin: Pin) -> &Pin {
        // Store the pin token
        let token = pin.token();

        // Insert the pin, and return a reference
        self.pins.insert(token, pin);
        self.pin(token).unwrap()
    }

    /// Get a registered pin by it's pin token.
    /// `None` is returned if the pin couldn't be found.
    pub fn pin(&self, token: PinToken) -> Option<&Pin> {
        self.pins.get(&token)
    }

    /// Get a mutable registered pin by it's pin token.
    /// `None` is returned if the pin couldn't be found.
    pub fn pin_mut(&mut self, token: PinToken) -> Option<&mut Pin> {
        self.pins.get_mut(&token)
    }

    /// Generate a new unique pin token, that can be used to identify a new pin.
    pub fn generate_pin_token(&mut self) -> PinToken {
        // Get the token index
        let token = self.token_index;

        // Increase the token value, and return the token
        self.token_index += 1;

        PinToken::new(token)
    }

//    /// Register a new pin to start polling on.
//    pub fn add_pin(&mut self, pin: &'a Pin) {
//        // The pin must not be in the list, then add the pin to the list
//        if !self.is_pin(&pin) {
//            self.pins.push(&pin);
//        }
//    }
//
//    /// Check whether the given pin is registered for polling.
//    /// `true` is returned if the pin is registered, `false` if it isn't.
//    pub fn is_pin(&self, pin: &'a Pin) -> bool {
//        self.pins.contains(&pin)
//    }
//
//    /// Deregister the given pin to stop polling on it.
//    /// Returns `true` if any pin was unregistered, `false` if no pin was unregistered.
//    // TODO: Test this method, to ensure it works properly.
//    pub fn remove_pin(&mut self, pin: &'a Pin) -> bool {
//        *&self.pins.iter()
//            .position(|e| e == &pin)
//            .map(|e| self.pins.remove(e))
//            .is_some()
//    }
}