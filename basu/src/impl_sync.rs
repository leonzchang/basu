use crate::{error::BasuError, event::Event, Arc, EventBus, Handler, HandlerId, HashMap, Mutex};
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
use rayon::prelude::*;

/// Implement for event handler
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
pub trait Handle<T>: Send + Sync {
    /// Handle event which is published from `EventBus`
    fn handle(&self, event: &Event<T>) -> Result<(), BasuError>;
}

impl<T: Sync> EventBus<T> {
    /// Subscribe to an event type.
    /// It takes the event type as a string and a handler implementing the `Handle<T>` trait.
    /// The method returns a `HandlerId` that uniquely identifies the handler within the event bus.
    ///
    /// ## Example
    /// ```no_run
    /// struct MyEventData {
    ///    // Define your event data structure here
    /// }
    ///
    /// struct MyEventHandler;
    ///
    /// impl Handle<MyEventData> for MyEventHandler {
    ///     fn handle(&self, event: &Event<MyEventData>) -> Result<(), BasuError> {
    ///         // Handle the event here
    ///         // ...
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let event_bus = EventBus::<MyEventData>::new();
    /// let handler = MyEventHandler;
    ///
    /// let handler_id = event_bus.subscribe("my_event", Box::new(handler))?;
    /// ```
    #[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
    pub fn subscribe(&self, event_type: &str, handler: Handler<T>) -> Result<HandlerId, BasuError> {
        let mut event_handler_map = self
            .event_handler_map
            .lock()
            .map_err(|_| BasuError::MutexPoisoned)?;

        match event_handler_map.get(event_type) {
            Some(handler_map) => {
                let mut handler_map = handler_map.lock().map_err(|_| BasuError::MutexPoisoned)?;
                let handler_id = HandlerId::new();
                handler_map.insert(handler_id.clone(), handler);

                Ok(handler_id)
            }
            None => {
                let mut handler_map = HashMap::new();
                let handler_id = HandlerId::new();
                handler_map.insert(handler_id.clone(), handler);

                event_handler_map.insert(event_type.to_owned(), Arc::new(Mutex::new(handler_map)));

                Ok(handler_id)
            }
        }
    }

    /// Unsubscribe handler from an event type.
    /// It takes the event type and the `HandlerId` of the handler to be removed.
    ///
    /// ```no_run
    /// struct MyEventData {
    ///    // Define your event data structure here
    /// }
    ///
    /// struct MyEventHandler;
    ///
    /// impl Handle<MyEventData> for MyEventHandler {
    ///     fn handle(&self, event: &Event<MyEventData>) -> Result<(), BasuError> {
    ///         // Handle the event here
    ///         // ...
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let event_bus = EventBus::<MyEventData>::new();
    /// let handler = MyEventHandler;
    /// let handler_id = event_bus.subscribe("my_event", Box::new(handler))?;
    ///
    /// event_bus.unsubscribe("my_event", &handler_id)?;
    /// ```
    #[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
    pub fn unsubscribe(&self, event_type: &str, handler_id: &HandlerId) -> Result<(), BasuError> {
        let event_handler_map = self
            .event_handler_map
            .lock()
            .map_err(|_| BasuError::MutexPoisoned)?;

        match event_handler_map.get(event_type) {
            Some(handler_map) => {
                let mut handler_map = handler_map.lock().map_err(|_| BasuError::MutexPoisoned)?;
                handler_map.remove(handler_id);

                Ok(())
            }

            None => Err(BasuError::EventTypeNotFOUND),
        }
    }

    /// Publish an event to subscribed handlers,
    /// It takes the event type and an `Event<T>` instance containing the event data.
    ///
    /// ```no_run
    /// struct MyEventData {
    ///    // Define your event data structure here
    /// }
    ///
    /// struct MyEventHandler;
    ///
    /// impl Handle<MyEventData> for MyEventHandler {
    ///     fn handle(&self, event: &Event<MyEventData>) -> Result<(), BasuError> {
    ///         // Handle the event here
    ///         // ...
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let event_bus = EventBus::<MyEventData>::new();
    /// let handler = MyEventHandler;
    /// let handler_id = event_bus.subscribe("my_event", Box::new(handler))?;
    /// let event_data = MyEventData { /* initialize your event data */ };
    /// let event = Event::new(event_data);
    ///
    /// event_bus.publish("my_event", &event)?;
    /// ```
    #[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
    pub fn publish(&self, event_type: &str, event_data: &Event<T>) -> Result<(), BasuError> {
        let event_handler_map = self
            .event_handler_map
            .lock()
            .map_err(|_| BasuError::MutexPoisoned)?;

        match event_handler_map.get(event_type) {
            Some(handler_map) => {
                let handler_map = handler_map.lock().map_err(|_| BasuError::MutexPoisoned)?;
                handler_map
                    .par_iter()
                    .try_for_each(|(_id, h)| h.handle(event_data))?;
                Ok(())
            }
            None => Err(BasuError::EventTypeNotFOUND),
        }
    }

    /// List all registered event types.
    /// It returns a Vec that contains the names of the registered event types.
    ///
    /// ```no_run
    /// struct MyEventData {
    ///    // Define your event data structure here
    /// }
    ///
    /// struct MyEventHandler;
    ///
    /// impl Handle<MyEventData> for MyEventHandler {
    ///     fn handle(&self, event: &Event<MyEventData>) -> Result<(), BasuError> {
    ///         // Handle the event here
    ///         // ...
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let event_bus = EventBus::<MyEventData>::new();
    /// let handler = MyEventHandler;
    /// let _handler_id = event_bus.subscribe("my_event", Box::new(handler))?;
    ///
    /// let event_types = event_bus.list()?;
    /// for event_type in event_types {
    ///     println!("Registered event type: {}", event_type);
    /// }
    ///```
    #[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
    pub fn list(&self) -> Result<Vec<String>, BasuError> {
        let event_handler_map = self
            .event_handler_map
            .lock()
            .map_err(|_| BasuError::MutexPoisoned)?;

        let event_types = event_handler_map.keys().cloned().collect();
        Ok(event_types)
    }

    /// Get the number of registered handlers for a specific event type.
    ///
    /// ```no_run
    /// struct MyEventData {
    ///    // Define your event data structure here
    /// }
    ///
    /// let event_bus = EventBus::<EventData>::new();
    ///
    /// let event_type = "my_event";
    /// let handler_count = event_bus.get_handler_count(event_type)?;
    ///
    /// println!("Number of handlers for event '{}': {}", event_type, handler_count);
    /// ```
    #[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
    pub fn get_handler_count(&self, event_type: &str) -> Result<usize, BasuError> {
        let event_handler_map = self
            .event_handler_map
            .lock()
            .map_err(|_| BasuError::MutexPoisoned)?;

        match event_handler_map.get(event_type) {
            Some(handler_map) => {
                let handler_map = handler_map.lock().map_err(|_| BasuError::MutexPoisoned)?;
                Ok(handler_map.len())
            }
            None => Err(BasuError::EventTypeNotFOUND),
        }
    }

    /// Clear all event handlers from the event bus.
    /// It removes all registered event handlers.
    ///
    /// ```no_run
    /// struct MyEventData {
    ///    // Define your event data structure here
    /// }
    ///
    /// struct MyEventHandler;
    ///
    /// impl Handle<MyEventData> for MyEventHandler {
    ///     fn handle(&self, event: &Event<MyEventData>) -> Result<(), BasuError> {
    ///         // Handle the event here
    ///         // ...
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let event_bus = EventBus::<MyEventData>::new();
    /// let handler = MyEventHandler;
    /// let _handler_id = event_bus.subscribe("my_event", Box::new(handler))?;
    ///
    /// event_bus.clear()?;
    ///
    /// println!("All event handlers cleared");
    /// ```
    ///
    /// **Note:** The `clear` method removes all event handlers and makes the event bus empty.
    #[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
    pub fn clear(&self) -> Result<(), BasuError> {
        let mut event_handler_map = self
            .event_handler_map
            .lock()
            .map_err(|_| BasuError::MutexPoisoned)?;

        event_handler_map.clear();
        Ok(())
    }
}
