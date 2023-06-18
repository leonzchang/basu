/// Errors which can occur when interacting with `EventBus`.
#[derive(thiserror::Error, Debug)]
pub enum BasuError {
    /// Sync mutex lock is poisoned.
    #[error("Mutex is poisoned")]
    MutexPoisoned,

    /// Event type not found in `EventBus`.
    #[error("event type not found")]
    EventTypeNotFOUND,

    /// Error occurs when `Handler` processing event.
    #[error(transparent)]
    HandlerError(#[from] anyhow::Error),
}
