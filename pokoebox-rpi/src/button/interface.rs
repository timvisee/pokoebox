use super::adapter::{self, Adapter};

/// Button interface.
pub struct Interface {
    /// Adapter to access buttons.
    adapter: Box<dyn Adapter>,
}

impl Interface {
    /// Construct new interface.
    pub fn new() -> Result<Self, Error> {
        let mut interface = Self {
            adapter: adapter::select_adapter().map_err(Error::Adapter)?,
        };

        // Set-up configured buttons
        interface.setup_buttons()?;

        Ok(interface)
    }

    /// Set up all configured buttons.
    fn setup_buttons(&mut self) -> Result<(), Error> {
        // Loop through button list, set-up each one
        for (button, config) in super::BUTTONS.iter() {
            self.adapter
                .setup_button(
                    *config,
                    Box::new(move |event| {
                        info!("Button event: {:?} -> {:?}", button, event);
                    }),
                )
                .map_err(Error::Adapter)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    /// Adapter error.
    Adapter(adapter::Error),
}
