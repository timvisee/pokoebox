use std::sync::{
    mpsc::{self, Receiver, Sender},
    {Arc, Mutex},
};

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
    /// Send given item through pipe.
    pub fn send(&self, item: T) -> Result<(), Error> {
        let mut out = self.inner.out.lock().expect("failed to obtain pipe lock");

        // Send through all outs, collect indices of disconnected outs and remove them
        out.iter()
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
                out.remove(i);
            });

        if out.is_empty() {
            Err(Error::NoReceiver)
        } else {
            Ok(())
        }
    }

    /// Register new listener.
    pub fn listen(&self) -> Receiver<T> {
        let (tx, rx) = mpsc::channel();
        self.inner
            .out
            .lock()
            .expect("failed to obtain pipe lock")
            .push(tx);

        // TODO: remove after debugging
        info!("Connected new pipe listener!");

        rx
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
    /// All channels going out.
    out: Mutex<Vec<Sender<T>>>,
}

impl<T> Default for InnerPipe<T>
where
    T: Clone + Send,
    Self: Send + Sync,
{
    fn default() -> Self {
        Self {
            out: Mutex::new(Vec::new()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// There is no connected receiver.
    NoReceiver,
}
