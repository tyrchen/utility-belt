mod broadcaster;

use std::sync::Arc;
use tokio::sync::mpsc;

pub use broadcaster::Broadcaster;

pub trait Topic<T>: Clone + Send + Sync + 'static {
    /// subscribe to a topic (here name should be session id)
    fn subscribe(self, name: impl Into<String>) -> (u32, mpsc::Receiver<Arc<T>>);
    /// unsubscribe to a topic
    fn unsubscribe(self, name: impl AsRef<str>, id: u32) -> Option<u32>;
    /// public a message to a topic
    fn publish(self, name: impl Into<String>, value: Arc<T>);
}

pub trait TopicId {
    fn get_topic_id(&self) -> String;
}
