//! # A Tour of Basu
//!
//! Basu is a crate designed to enable event-driven architectures in Rust applications
//! with ease and flexibility.
//!
//! ### Features:
//!
//! - Support for both asynchronous and synchronous event handling. Choose the approach
//!  that best fits your application's needs.
//!
//! - Provides an abstraction for representing events, allowing you to define custom
//!  event structures with payload data.
//!
//! - Well-defined error types and error handling mechanisms for reliable event bus
//!  operations.
//!
//! To enable the synchronous event handling capability, use the `sync` feature:
//!
//! ```toml
//! [dependencies]
//! basu = { version = "0.1.0", features = ["sync"] }
//! ```
//!
//! To enable the asynchronous event handling capability, use the `async` feature:
//!
//! ```toml
//! [dependencies]
//! basu = { version = "0.1.0", features = ["async"] }
//! ```
#![deny(missing_docs)]

#[cfg(not(any(feature = "async", feature = "sync")))]
compile_error!("Either `async` or `sync` feature must be enabled");

#[cfg(all(feature = "async", feature = "sync"))]
compile_error!("The `async` and `sync` features cannot be enabled simultaneously");

/// basu error
pub mod error;
/// basu event
pub mod event;
#[cfg(feature = "async")]
mod impl_async;
#[cfg(feature = "sync")]
mod impl_sync;
#[cfg(test)]
mod tests;

#[cfg(feature = "async")]
pub use async_trait::async_trait;
#[cfg(feature = "async")]
pub use impl_async::Handle;
#[cfg(feature = "sync")]
pub use impl_sync::Handle;
#[cfg(feature = "sync")]
use std::sync::Mutex;
#[cfg(feature = "async")]
use tokio::sync::Mutex;

use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

/// Hanlder
pub type Handler<T> = Box<dyn Handle<T>>;
/// Hanlder map with Id
pub type HandlerMap<T> = Arc<Mutex<HashMap<HandlerId, Handler<T>>>>;
/// Event Hanlder map
pub type EventHandlerMap<T> = Arc<Mutex<HashMap<String, HandlerMap<T>>>>;

/// An asynchronous `EventBus` to interact with.
pub struct EventBus<T> {
    event_handler_map: EventHandlerMap<T>,
}

impl<T> EventBus<T> {
    /// create a new `EventBus`
    pub fn new() -> Self {
        Self {
            event_handler_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

/// HandlerId is the key in `HandlerMap` hash map.
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct HandlerId {
    id: Uuid,
}

impl HandlerId {
    /// create a new `HandlerId`
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}
