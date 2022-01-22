use dashmap::{DashMap, DashSet};
use std::{
    fmt,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};
use tokio::sync::mpsc;
use tracing::{info, warn};

use crate::Topic;

/// max data inside a topic
const BROADCAST_CAPACITY: usize = 128;

/// next subscription id
static NEXT_ID: AtomicU32 = AtomicU32::new(1);

/// get next subscription id
fn get_next_subscription_id() -> u32 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

pub struct Broadcaster<T> {
    topics: DashMap<String, DashSet<u32>>,
    subscriptions: DashMap<u32, mpsc::Sender<Arc<T>>>,
}

impl<T> Default for Broadcaster<T> {
    fn default() -> Self {
        Self {
            topics: DashMap::new(),
            subscriptions: DashMap::new(),
        }
    }
}

// Note: here we implement Topic for Arc<Broadcaster>, not Broadcaster
impl<T> Topic<T> for Arc<Broadcaster<T>>
where
    T: fmt::Debug + Send + Sync + 'static,
{
    fn subscribe(self, name: impl Into<String>) -> (u32, mpsc::Receiver<Arc<T>>) {
        let id = {
            let entry = self.topics.entry(name.into()).or_default();
            let id = get_next_subscription_id();
            entry.value().insert(id);
            id
        };

        let (tx, rx) = mpsc::channel(BROADCAST_CAPACITY);

        self.subscriptions.insert(id, tx);
        info!("Subscription {} is added", id);

        (id, rx)
    }

    fn unsubscribe(self, name: impl AsRef<str>, id: u32) -> Option<u32> {
        self.remove_subscription(name, id)
    }

    fn publish(self, name: impl Into<String>, value: Arc<T>) {
        let name = name.into();
        tokio::spawn(async move {
            let mut ids = vec![];
            if let Some(topic) = self.topics.get(&name) {
                // clone all subscription id for an entire topicid
                // normally a topic should have a few subscriptions, here even
                // if a topic got 10k subscription, there's only 40k heap memory,
                // so not a big deal.

                let subscriptions = topic.value().clone();
                drop(topic);

                for id in subscriptions.into_iter() {
                    if let Some(tx) = self.subscriptions.get(&id) {
                        if let Err(e) = tx.send(value.clone()).await {
                            warn!("Publish to {} failed! error: {:?}", id, e);
                            // client disconnected
                            ids.push(id);
                        }
                    }
                }
            }

            for id in ids {
                self.remove_subscription(&name, id);
            }
        });
    }
}

impl<T> Broadcaster<T> {
    #[allow(dead_code)]
    pub fn topics(&self) -> &DashMap<String, DashSet<u32>> {
        &self.topics
    }

    pub fn remove_subscription(&self, name: impl AsRef<str>, id: u32) -> Option<u32> {
        let name = name.as_ref();
        if let Some(v) = self.topics.get_mut(name) {
            v.remove(&id);

            if v.is_empty() {
                info!("Topic: {:?} is deleted", name);
                drop(v);
                self.topics.remove(name);
            }
        }

        info!("Subscription {} is removed!", id);
        // delete in subscription table
        self.subscriptions.remove(&id).map(|(id, _)| id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::time::Duration;
    use tokio::time;

    #[tokio::test]
    async fn publish_should_work() {
        let topic = "lobby";
        let broadcaster: Arc<Broadcaster<String>> = Arc::new(Broadcaster::default());
        let (id, mut receiver) = broadcaster.clone().subscribe(topic);
        broadcaster.publish(topic, Arc::new("hello world".into()));
        let data = receiver.recv().await.unwrap();
        assert!(id > 0);
        assert_eq!(data.as_ref(), "hello world");
    }

    #[tokio::test]
    async fn subscribe_abnormal_quit_should_be_removed_on_next_publish() {
        let topic = "lobby";
        let broadcaster: Arc<Broadcaster<String>> = Arc::new(Broadcaster::default());
        let id = {
            let (id, receiver) = broadcaster.clone().subscribe(topic);
            drop(receiver);
            id as u32
        };

        // upon publish, subscription is invalid so it will be removed
        broadcaster
            .clone()
            .publish(topic, Arc::new("hello world".into()));
        time::sleep(Duration::from_millis(10)).await;

        // if tried to unsubscribe, a Wormhole NotFound error will be returned
        let result = broadcaster.unsubscribe(topic, id);
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn unsubscribe_should_work() -> Result<()> {
        let topic = "lobby";
        let broadcaster: Arc<Broadcaster<String>> = Arc::new(Broadcaster::default());
        let (id1, _receiver) = broadcaster.clone().subscribe(topic);
        let (id2, _receiver) = broadcaster.clone().subscribe(topic);
        let removed_id1 = broadcaster.clone().unsubscribe(topic, id1).unwrap();
        assert_eq!(id1, removed_id1);
        // there's still one subscription alive for topic, so topic exists
        assert!(broadcaster.topics().contains_key(topic));
        let removed_id2 = broadcaster.clone().unsubscribe(topic, id2).unwrap();
        assert_eq!(id2, removed_id2);

        // now no subscriptions
        assert!(!broadcaster.topics().contains_key(topic));

        Ok(())
    }

    #[tokio::test]
    async fn unsubscribe_random_id_should_error() {
        let topic = "lobby";
        let broadcaster: Arc<Broadcaster<String>> = Arc::new(Broadcaster::default());
        let (_id, _receiver) = broadcaster.clone().subscribe(topic);
        let result = broadcaster.unsubscribe(topic, 9527);
        assert!(result.is_none());
    }
}
