use crate::errors::{Error, Result};
use std::any::type_name;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::SendError;
use tokio::sync::broadcast::{Receiver, Sender};

pub struct TokioSender<M: prost::Message + Clone> {
    tx: Sender<M>,
    stop: Arc<AtomicBool>,
}

impl<M: prost::Message + Clone> TokioSender<M> {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self {
            tx,
            stop: Default::default(),
        }
    }

    #[inline]
    pub fn send(&self, event: M) -> Result<()> {
        if self.stop.load(Ordering::Relaxed) {
            tracing::warn!("cluster event sender has been stopped fail.");
            Err(Error::Closed)
        } else if let Err(e) = self.tx.send(event) {
            tracing::warn!("QueueSender send fail, {}", e);
            Err(Error::Send)
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.tx.is_empty()
    }
}

pub struct TokioReceiver<M: prost::Message + Clone> {
    rx: Receiver<M>,
    selector: fn(M) -> Option<M>,
}

impl<M: prost::Message + Clone> TokioReceiver<M> {
    pub(crate) fn from_sender(sender: &TokioSender<M>, selector: fn(M) -> Option<M>) -> Self {
        Self {
            rx: sender.tx.subscribe(),
            selector,
        }
    }

    #[inline]
    pub async fn recv_mut(&mut self) -> Result<Option<M>> {
        match self.rx.recv().await {
            Ok(c) => Ok((self.selector)(c)),
            Err(e) => {
                if let broadcast::error::RecvError::Lagged(lag) = e {
                    tracing::warn!("TokioSubscriber[{}] is lagged: {}", type_name::<M>(), lag);
                    Ok(None)
                } else {
                    Err(Error::Closed)
                }
            }
        }
    }
}
