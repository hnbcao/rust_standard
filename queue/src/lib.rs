pub mod broadcast;
pub mod cluster_data;
pub mod cluster_event;
pub mod errors;
pub mod message;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::broadcast::TokioSender;
    use crate::cluster_event::ClusterEventSender;
    use crate::message::event::cluster_event_proto::ClusterEvent;
    use crate::message::event::{ClusterEventProto, UserEvent};
    use std::time::Duration;

    #[tokio::test]
    pub async fn test_queue() {
        tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).with_test_writer().init();
        let sender = ClusterEventSender::Queue(TokioSender::new(2));
        for i in 0..2 {
            let mut subscriber = sender.subscribe().unwrap();
            tokio::spawn(async move {
                while let Ok(s) = subscriber.recv_mut().await {
                    println!("Received[{}]: {:?}", i, s);
                }
            });
        }
        tracing::debug!("subscriber test");
        for i in 0..10 {
            sender
                .send(ClusterEventProto {
                    ts: i,
                    cluster_event: Some(ClusterEvent::UserEvent(UserEvent {})),
                })
                .expect("TODO: panic message");
        }
        sender.stop();
        while !sender.is_empty() {
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
}
