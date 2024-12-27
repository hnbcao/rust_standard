use crate::broadcast::{TokioReceiver, TokioSender};
use crate::errors::Result;
use crate::message::event::ClusterEventProto;

pub enum ClusterEventSender {
    Queue(TokioSender<ClusterEventProto>),
}

impl ClusterEventSender {
    #[inline]
    pub fn send(&self, event: ClusterEventProto) -> Result<()> {
        tracing::info!("sending cluster event: {:?}.", event);
        match self {
            ClusterEventSender::Queue(sender) => sender.send(event),
        }
    }

    #[inline]
    pub fn subscribe(&self) -> Result<ClusterEventReceiver> {
        tracing::info!("subscribing cluster event receiver.");
        let receiver = match self {
            ClusterEventSender::Queue(sender) => ClusterEventReceiver::Queue(TokioReceiver::from_sender(sender, Some)),
        };
        Ok(receiver)
    }

    #[inline]
    pub fn stop(&self) {
        tracing::info!("stopping cluster event sender.");
        match self {
            ClusterEventSender::Queue(sender) => sender.stop(),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            ClusterEventSender::Queue(sender) => sender.is_empty(),
        }
    }
}

pub enum ClusterEventReceiver {
    Queue(TokioReceiver<ClusterEventProto>),
}

impl ClusterEventReceiver {
    #[inline]
    pub async fn recv_mut(&mut self) -> Result<Option<ClusterEventProto>> {
        match self {
            Self::Queue(q) => q.recv_mut().await,
        }
    }
}
