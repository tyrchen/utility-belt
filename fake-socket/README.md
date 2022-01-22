# Fake Socket

Usage:

```rust
use tokio::sync::mpsc;
use axum::extract::ws::Message;

let (_tx1, rx1) = mpsc::unbounded_channel();
let (tx2, mut rx2) = mpsc::unbounded_channel();

let socket = FakeSocket::<Message, axum::Error>::new(rx1, tx2);
// use it as a websocket in your test
```
