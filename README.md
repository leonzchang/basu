# Basu
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/leonzchang/basu/blob/main/LICENSE)
[![crates.io](https://img.shields.io/crates/v/basu)](
https://crates.io/crates/basu)

A flexible event bus crate enables decoupling between different components of an application through the event-driven pattern.

Basu provides support for both asynchronous and synchronous event handling.

## Example

### Sync
To use sync Basu, make sure you disable `default-features` of the basu crate on `Cargo.toml`:

```toml
[dependencies]
basu = { version = "0.1", default-features = false, features = ["sync"] }
```

### Async
To use async Basu in your Rust project, add the following line to `Cargo.toml`:

```toml
[dependencies]
basu = "0.1"
```

###  Usage:
```rust
use basu::{BasuError, Event, EventBus, Handle};


struct MyEventHandler;

// Implement the Handle trait for your event handler structs
impl Handle<MyEvent> for MyEventHandler {
    async fn handle(&self, event: &Event<MyEvent>) -> Result<(), BasuError> {
        // Handle the event logic here
        Ok(())
    }
}

struct Data {
    message: String
}

#[tokio::main]
async fn main() {
    let event_bus = EventBus::new();

    let eveny_type = "my_event";
    // Subscribe to events by providing an event name and the corresponding event handler
    let handler = Box::new(MyEventHandler);
    let handler_id = event_bus.subscribe(eveny_type, handler)?;

    // Publish events
    let event = Event::new(Data {
        message: "hello".to_owned(),
    });
    event_bus.publish(eveny_type, &event)?;

    // Unsubscribe from events
    event_bus.unsubscribe(eveny_type, &handler_id)?;
}
```

## License
This project is licensed under the [MIT license](https://github.com/leonzchang/basu/blob/main/LICENSE).