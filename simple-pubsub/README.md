# Simple Pubsub

Usage:

```rust
let topic = "lobby";
let broadcaster: Arc<Broadcaster<String>> = Arc::new(Broadcaster::default());
let (id, mut receiver) = broadcaster.clone().subscribe(topic);
broadcaster.publish(topic, Arc::new("hello world".into()));
let data = receiver.recv().await.unwrap();
assert!(id > 0);
assert_eq!(data.as_ref(), "hello world");
```
