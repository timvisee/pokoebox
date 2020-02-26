use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use rppal::gpio::{self, Gpio, InputPin};

use super::{ButtonConfig, Error, Event};

/// Button debounce time.
pub const BUTTON_DEBOUNCE_TIME: Duration = Duration::from_millis(1000 / 24);

/// Adapter. Talks to buttons.
pub struct Adapter {
    /// GPIO interface to access buttons.
    gpio: Gpio,
}

impl Adapter {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            gpio: Gpio::new().map_err(|_| Error::Adapter)?,
        })
    }

    /// Allocate a GPIO input pin.
    fn allocate_input(&self, pin: u8, pullup: bool) -> Result<InputPin, Error> {
        let pin = self.gpio.get(pin).map_err(|_| Error::Adapter)?;
        Ok(if pullup {
            pin.into_input_pullup()
        } else {
            pin.into_input()
        })
    }
}

impl super::Adapter for Adapter {
    fn setup_button(
        &mut self,
        button: ButtonConfig,
        mut callback: Box<dyn FnMut(Event) + Send + 'static>,
    ) -> Result<(), Error> {
        // Track buton state
        let state = ButtonState::default();

        // Set-up button
        match button {
            ButtonConfig::Push(pin) => {
                let mut input = self.allocate_input(pin, false)?;
                let callback_state = state.clone();
                input
                    .set_async_interrupt(gpio::Trigger::RisingEdge, move |level| match level {
                        gpio::Level::High => {
                            if callback_state.update_state(1) {
                                callback(Event::Press);
                            }
                        }
                        gpio::Level::Low => {
                            callback_state.update_state(0);
                        }
                    })
                    .map_err(|_| Error::Adapter)?;
                state.get_mut().pins.push(input);
            }
            ButtonConfig::Rotary(pin_a, pin_b) => {
                let mut input_a = self.allocate_input(pin_a, true)?;
                let input_b = self.allocate_input(pin_b, true)?;
                let callback_state = state.clone();
                input_a
                    .set_async_interrupt(gpio::Trigger::Both, move |level_a| {
                        let level_b = callback_state.get_mut().pins[1].read();
                        if callback_state.update_state(0) {
                            callback(if level_a == level_b {
                                Event::Down
                            } else {
                                Event::Up
                            });
                        }
                    })
                    .map_err(|_| Error::Adapter)?;
                state.get_mut().pins.append(&mut vec![input_a, input_b]);
            }
        }

        Ok(())
    }
}

/// The button state.
#[derive(Default, Clone)]
pub struct ButtonState {
    inner: Arc<Mutex<InnerButtonState>>,
}

impl ButtonState {
    /// Get the mutable inner button state.
    pub fn get_mut(&self) -> MutexGuard<InnerButtonState> {
        self.inner
            .lock()
            .expect("failed to lock inner button state")
    }

    /// Update button state after debounce timer.
    ///
    /// This method updates the internal `state`, but only if the button debounce timer has passed.
    /// True is returned if the state was updated, false if it wasn't because of the debounce
    /// timer.
    fn update_state(&self, state: u8) -> bool {
        let result = self.get_mut().update_state(state);

        // TODO: remove after debugging
        debug!("Update button state: {} (debounced: {})", state, !result);

        result
    }
}

/// Inner button state.
pub struct InnerButtonState {
    /// Last known button state.
    state: u8,

    /// Last time button was triggered.
    last: Instant,

    /// List of GPIO input pins.
    pins: Vec<InputPin>,
}

impl InnerButtonState {
    /// Update button state after debounce timer.
    ///
    /// This method updates the internal `state`, but only if the button debounce timer has passed.
    /// True is returned if the state was updated, false if it wasn't because of the debounce
    /// timer.
    fn update_state(&mut self, state: u8) -> bool {
        // Button debounce timer must have passed
        if self.last.elapsed() < BUTTON_DEBOUNCE_TIME {
            return false;
        }

        // Update state
        self.last = Instant::now();
        self.state = state;
        true
    }
}

impl Default for InnerButtonState {
    fn default() -> Self {
        Self {
            state: 0,
            last: Instant::now() - BUTTON_DEBOUNCE_TIME,
            pins: Vec::new(),
        }
    }
}

// #[derive(Debug)]
// pub enum Error {
//     /// While sending data, only part of the data was sent.
//     SentPartial,

//     /// An i2c bus error.
//     I2c(i2c::Error),
// }
