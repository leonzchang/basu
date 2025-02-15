# Basu
<p align="center">
  <img src="https://raw.githubusercontent.com/leonzchang/basu/refs/heads/main/assets/basu.png" alt="basu">
</p>

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/leonzchang/basu/blob/main/LICENSE)
[![crates.io](https://img.shields.io/crates/v/basu)](
https://crates.io/crates/basu)
[![docs.rs](https://img.shields.io/badge/docs-docs.rs-green)](https://docs.rs/basu)

A flexible event bus crate enables decoupling between different components of an application through the event-driven pattern.

Basu provides support for both asynchronous and synchronous event handling.

## Example

### Feature
- Sync:
    - To use sync Basu, make sure you disable `default-features` of the basu crate on `Cargo.toml`:
        ```toml
        [dependencies]
        basu = { version = "0.1", default-features = false, features = ["sync"] }
        ```

- Async:
    - To use async Basu in your Rust project, add the following line to `Cargo.toml`:
        ```toml
        [dependencies]
        basu = "0.1"
        ```

###  Usage:
To run the example, add the following line to `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1", default-features = false, features = [ "macros", "rt-multi-thread"] }
basu = "0.1"
```

Then, on your `main.rs`:
```rust
use basu::{async_trait, error::BasuError, event::Event, EventBus, Handle};

#[derive(Debug)]
struct Data {
    message: String,
}

struct MyEventHandler;

// Implement the Handle trait for your event handler structs
#[async_trait]
impl Handle<Data> for MyEventHandler {
    async fn handle(&self, event: &Event<Data>) -> Result<(), BasuError> {
        // Handle the event logic here
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), BasuError> {
    let event_bus = EventBus::new();

    let eveny_type = "my_event";
    // Subscribe to events by providing an event name and the corresponding event handler
    let handler = Box::new(MyEventHandler);
    let handler_id = event_bus.subscribe(eveny_type, handler).await;

    // Publish events
    let event = Event::new(Data {
        message: "hello".to_owned(),
    });
    event_bus.publish(eveny_type, &event).await?;

    // Unsubscribe from events
    event_bus.unsubscribe(eveny_type, &handler_id).await
}
```

## License
This project is licensed under the [MIT license](https://github.com/leonzchang/basu/blob/main/LICENSE).