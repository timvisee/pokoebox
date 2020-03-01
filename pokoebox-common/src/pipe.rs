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
    pub fn send(&self, item: T) -> Result<(), Error> {
        // Send through channels and callbacks
        let result_channels = self.send_channels(item.clone());
        let result_callbacks = self.send_callbacks(item);

        match (result_channels, result_callbacks) {
            (Err(Error::NoReceiver), Err(Error::NoReceiver)) => Err(Error::NoReceiver),
            #[allow(unreachable_patterns)]
            (Err(err), Err(Error::NoReceiver))
            | (Err(Error::NoReceiver), Err(err))
            | (Err(err), Ok(_))
            | (Ok(_), Err(err)) => Err(err),
            (Ok(_), Ok(_)) => Ok(()),
        }
    }

    fn send_channels(&self, item: T) -> Result<(), Error> {
        let mut receivers = self
            .inner
            .receivers
            .lock()
            .expect("failed to obtain pipe lock");

        // Send through all channels, collect indices of disconnected channels and remove them
        receivers
            .iter()
            .enumerate()
            .filter_map(|(i, tx)| {
                let result = tx.send(item.clone());

                // TODO: remove this after debugging
                info!("InnerPipe send result: {:?}", result);

                result.err().map(|_| i)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .for_each(|i| {
                receivers.remove(i);
            });

        if receivers.is_empty() {
            Err(Error::NoReceiver)
        } else {
            Ok(())
        }
    }

    fn send_callbacks(&self, item: T) -> Result<(), Error> {
        let mut callbacks = self
            .inner
            .callbacks
            .lock()
            .expect("failed to obtain pipe lock");

        // Send through all callbacks
        for callback in callbacks.iter_mut() {
            callback(item.clone());
        }

        if callbacks.is_empty() {
            Err(Error::NoReceiver)
        } else {
            Ok(())
        }
    }

    /// Allocate new listener.
    pub fn listen(&self) -> Receiver<T> {
        let (tx, rx) = mpsc::channel();
        self.inner
            .receivers
            .lock()
            .expect("failed to obtain pipe lock")
            .push(tx);

        // TODO: remove after debugging
        info!("Connected new pipe listener!");

        rx
    }

    /// Register a callback.
    pub fn register_callback<C>(&self, callback: C)
    where
        C: FnMut(T) + Send + 'static,
    {
        self.inner
            .callbacks
            .lock()
            .expect("failed to obtain pipe lock")
            .push(Box::new(callback));
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
        }
    }
}

/// Pipe error.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// No connected receiver (callback, channel), event was not sent at all.
    NoReceiver,
}
