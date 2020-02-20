use glib::Sender;
use std::sync::Mutex;

/// Pages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageType {
    Launchpad,
    Volume,
    Test,
}

pub struct PageController {
    channel: Mutex<Option<Sender<PageType>>>,
}

impl PageController {
    pub fn new() -> Self {
        Self {
            channel: Mutex::new(None),
        }
    }

    pub fn set_channel(&self, channel: Sender<PageType>) {
        self.channel
            .lock()
            .expect("failed to lock page controller channel")
            .replace(channel);
    }

    pub fn goto_page(&self, page: PageType) -> std::result::Result<(), ()> {
        // Obtain channel lock
        let mut guard = self
            .channel
            .lock()
            .expect("failed to lock page controller channel");

        // Send request over channel
        match guard.as_ref() {
            Some(channel) => {
                if channel.send(page).is_ok() {
                    Ok(())
                } else {
                    guard.take();
                    Err(())
                }
            }
            None => Err(()),
        }
    }
}
