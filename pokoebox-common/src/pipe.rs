use std::sync::{
    mpsc::{self, Receiver, Sender},
    {Arc, Mutex},
};

/// Flexible channel-like listener system.
#[derive(Clone)]
pub struct Pipe<T>
where
    T: Clone + Send,
    Self: Send + Sync,
{
    inner: Arc<InnerPipe<T>>,
}

impl<T> Pipe<T>
where
    T: Clone + Send,
    Self: Send + Sync,
{
    /// Send item through pipe.
    ///
    /// Returns number of receivers this was passed onto.
    pub fn send(&self, item: T) -> Result<usize, Error> {
        // Send through channels and callbacks
        let receivers = self.send_channels(item.clone())? + self.send_callbacks(item.clone())?;

        // Queue item if there was no receiver
        if receivers == 0 {
            warn!("Sending through pipe, but there is no receiver, queueing now...");
            self.inner
                .queue
                .lock()
                .expect("failed to obtain pipe queue lock")
                .push(item);
        }

        Ok(receivers)
    }

    fn send_channels(&self, item: T) -> Result<usize, Error> {
        let mut receivers = self
            .inner
            .receivers
            .lock()
            .expect("failed to obtain pipe lock");

        // Send through all channels, collect indices of disconnected channels and remove them
        receivers
            .iter()
            .enumerate()
            .filter_map(|(i, tx)| tx.send(item.clone()).err().map(|_| i))
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .for_each(|i| {
                receivers.remove(i);
            });

        Ok(receivers.len())
    }

    fn send_callbacks(&self, item: T) -> Result<usize, Error> {
        let mut callbacks = self
            .inner
            .callbacks
            .lock()
            .expect("failed to obtain pipe lock");

        // Send through all callbacks
        for callback in callbacks.iter_mut() {
            callback(item.clone());
        }

        Ok(callbacks.len())
    }

    /// Allocate new listener.
    pub fn listen(&self) -> Receiver<T> {
        // Allocate channel, add to receivers
        let (tx, rx) = mpsc::channel();
        let mut receivers = self
            .inner
            .receivers
            .lock()
            .expect("failed to obtain pipe receivers lock");

        // Send pending queue items over new channel
        if receivers.is_empty() {
            self.inner
                .queue
                .lock()
                .expect("failed to obtain pipe queue lock")
                .drain(..)
                .for_each(|item| {
                    if let Err(err) = tx.send(item) {
                        error!("Failed to send pending queue item to receiver: {:?}", err);
                    }
                });
        }

        // Remember new channel
        receivers.push(tx);

        // TODO: remove after debugging
        debug!("Connected new pipe listener!");

        rx
    }

    /// Register a callback.
    pub fn register_callback<C>(&self, mut callback: C)
    where
        C: FnMut(T) + Send + 'static,
    {
        // Obtain callbacks list lock
        let mut callbacks = self
            .inner
            .callbacks
            .lock()
            .expect("failed to obtain pipe callbacks lock");

        // Send pending queue items over new callback
        if callbacks.is_empty() {
            self.inner
                .queue
                .lock()
                .expect("failed to obtain pipe queue lock")
                .drain(..)
                .for_each(|item| callback(item));
        }

        // Remember new callback
        callbacks.push(Box::new(callback));
    }
}

impl<T> Default for Pipe<T>
where
    T: Clone + Send,
    Self: Send + Sync,
{
    fn default() -> Self {
        Self {
            inner: Arc::new(InnerPipe::default()),
        }
    }
}

struct InnerPipe<T>
where
    T: Clone + Send,
    Self: Send + Sync,
{
    /// All receiving ends of allocated channels.
    receivers: Mutex<Vec<Sender<T>>>,

    /// Callbacks.
    callbacks: Mutex<Vec<Box<dyn FnMut(T) + Send>>>,

    /// Queued items not yet processed.
    queue: Mutex<Vec<T>>,
}

impl<T> Default for InnerPipe<T>
where
    T: Clone + Send,
    Self: Send + Sync,
{
    fn default() -> Self {
        Self {
            receivers: Mutex::new(Vec::new()),
            callbacks: Mutex::new(Vec::new()),
            queue: Mutex::new(Vec::new()),
        }
    }
}

/// Pipe error.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    // /// No connected receiver (callback, channel), event was not sent at all.
// TODO: make this obsolete?
// _NoReceiver,
}
