use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use rppal::gpio::{self, Gpio};

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

    /// Set-up a GPIO pin
    fn setup_gpio_input<C>(&self, pin: u8, callback: C) -> Result<(), Error>
    where
        C: FnMut(gpio::Level) + Send + 'static,
    {
        self.gpio
            .get(pin)
            .map_err(|_| Error::Adapter)?
            .into_input()
            .set_async_interrupt(gpio::Trigger::Both, callback)
            .map_err(|_| Error::Adapter)
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
                self.setup_gpio_input(pin, move |level| match level {
                    gpio::Level::High => {
                        if state.update_state(1) {
                            callback(Event::Press);
                        }
                    }
                    gpio::Level::Low => {
                        state.update_state(0);
                    }
                })?;
            }
            ButtonConfig::Rotary(_, _) => todo!("implement rotary encoder button setup"),
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
    /// Update button state after debounce timer.
    ///
    /// This method updates the internal `state`, but only if the button debounce timer has passed.
    /// True is returned if the state was updated, false if it wasn't because of the debounce
    /// timer.
    fn update_state(&self, state: u8) -> bool {
        let result = self
            .inner
            .lock()
            .expect("failed to lock inner button state")
            .update_state(state);

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
        self.state = state;
        true
    }
}

impl Default for InnerButtonState {
    fn default() -> Self {
        Self {
            state: 0,
            last: Instant::now() - BUTTON_DEBOUNCE_TIME,
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
