#![cfg(feature = "rpi")]

use std::sync::{Mutex, MutexGuard};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;

use super::cupi::CuPi;

use error::Error;
use result::Result;
use super::pin::Pin;
use super::pin_config::PinConfig;
use super::pin_token::PinToken;

pub struct PinAccessor<'a> {
    /// The mutex guard.
    guard: MutexGuard<'a, HashMap<PinToken, Pin>>,
}

impl<'a> PinAccessor<'a> {
    /// Construct a new accessor.
    pub fn new(guard: MutexGuard<'a, HashMap<PinToken, Pin>>) -> Self {
        PinAccessor {
            guard: guard,
        }
    }

    /// Add the given pin to the manager.
    /// A reference to the added pin is returned.
    pub fn add_pin(&mut self, pin: Pin) -> &Pin {
        // Store the pin token
        let token = pin.token();

        // Insert the pin, and return a reference
        self.guard.insert(token, pin);
        self.pin(token).unwrap()
    }

    /// Get a registered pin by it's pin token.
    /// `None` is returned if the pin couldn't be found.
    pub fn pin(&self, token: PinToken) -> Option<&Pin> {
        self.guard.get(&token)
    }

    /// Get a mutable registered pin by it's pin token.
    /// `None` is returned if the pin couldn't be found.
    pub fn pin_mut(&mut self, token: PinToken) -> Option<&mut Pin> {
        self.guard.get_mut(&token)
    }
}

/// GPIO manager.
pub struct GpioManager {
    /// CuPi instance, to create new pin instances.
    cupi: CuPi,

    /// List of pins that are instantiated.
    pins: HashMap<PinToken, Pin>,

    /// List of pins that are instantiated.
    pins_mutex: Mutex<HashMap<PinToken, Pin>>,

    /// Token index, used to create an unique auto incrementing token value.
    token_index: AtomicUsize,
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
            pins: HashMap::new(),
            pins_mutex: Mutex::new(HashMap::new()),
            token_index: AtomicUsize::new(0),
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
        PinAccessor::new(self.pins_mutex.lock().unwrap())
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

//    /// Add the given pin to the manager.
//    /// A reference to the added pin is returned.
//    pub fn add_pin(&mut self, pin: Pin) -> &Pin {
//        // Store the pin token
//        let token = pin.token();
//
//        // Insert the pin, and return a reference
//        self.pins.insert(token, pin);
//        self.pin(token).unwrap()
//    }
//
//    /// Get a registered pin by it's pin token.
//    /// `None` is returned if the pin couldn't be found.
//    pub fn pin(&self, token: PinToken) -> Option<&Pin> {
//        self.pins.get(&token)
//    }
//
//    /// Get a mutable registered pin by it's pin token.
//    /// `None` is returned if the pin couldn't be found.
//    pub fn pin_mut(&mut self, token: PinToken) -> Option<&mut Pin> {
//        self.pins.get_mut(&token)
//    }

    /// Generate a new unique pin token, that can be used to identify a new pin.
    pub fn generate_pin_token(&mut self) -> PinToken {
        // Generate a new token
        let token = PinToken::new(self.token_index.load(Ordering::Relaxed));

        // Increase the index by one for followup tokens
        self.token_index.fetch_add(1usize, Ordering::Relaxed);

        token
    }

    /// Poll the pins for signal changes.
    pub fn poll_pins(&self) {
        // TODO: Poll pins here!
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