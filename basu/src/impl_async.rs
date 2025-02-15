use crate::{
    async_trait, error::BasuError, event::Event, Arc, EventBus, Handler, HandlerId, HashMap, Mutex,
};

/// Implement for event handler
#[async_trait]
pub trait Handle<T>: Send + Sync {
    /// Handle event which is published from `EventBus`
    async fn handle(&self, event: &Event<T>) -> Result<(), BasuError>;
}

impl<T> EventBus<T> {
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
    /// #[async_trait]
    /// impl Handle<MyEventData> for MyEventHandler {
    ///     async fn handle(&self, event: &Event<MyEventData>) -> Result<(), BasuError> {
    ///         // Handle the event here
    ///         // ...
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let event_bus = EventBus::<MyEventData>::new();
    /// let handler = MyEventHandler;
    ///
    /// let handler_id = event_bus.subscribe("my_event", Box::new(handler)).await;
    /// ```
    pub async fn subscribe(&self, event_type: &str, handler: Handler<T>) -> HandlerId {
        let mut event_handler_map = self.event_handler_map.lock().await;

        match event_handler_map.get(event_type) {
            Some(handler_map) => {
                let mut handler_map = handler_map.lock().await;
                let handler_id = HandlerId::new();
                handler_map.insert(handler_id.clone(), handler);

                handler_id
            }
            None => {
                let mut handler_map = HashMap::new();
                let handler_id = HandlerId::new();
                handler_map.insert(handler_id.clone(), handler);

                event_handler_map.insert(event_type.to_owned(), Arc::new(Mutex::new(handler_map)));

                handler_id
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
    /// #[async_trait]
    /// impl Handle<MyEventData> for MyEventHandler {
    ///     async fn handle(&self, event: &Event<MyEventData>) -> Result<(), BasuError> {
    ///         // Handle the event here
    ///         // ...
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let event_bus = EventBus::<MyEventData>::new();
    /// let handler = MyEventHandler;
    /// let handler_id = event_bus.subscribe("my_event", Box::new(handler)).await;
    ///
    /// event_bus.unsubscribe("my_event", &handler_id).await?;
    /// ```
    pub async fn unsubscribe(
        &self,
        event_type: &str,
        handler_id: &HandlerId,
    ) -> Result<(), BasuError> {
        let event_handler_map = self.event_handler_map.lock().await;

        match event_handler_map.get(event_type) {
            Some(handler_map) => {
                let mut handler_map = handler_map.lock().await;
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
    /// #[async_trait]
    /// impl Handle<MyEventData> for MyEventHandler {
    ///     async fn handle(&self, event: &Event<MyEventData>) -> Result<(), BasuError> {
    ///         // Handle the event here
    ///         // ...
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let event_bus = EventBus::<MyEventData>::new();
    /// let handler = MyEventHandler;
    /// let handler_id = event_bus.subscribe("my_event", Box::new(handler)).await;
    /// let event_data = MyEventData { /* initialize your event data */ };
    /// let event = Event::new(event_data);
    ///
    /// event_bus.publish("my_event", &event).await?;
    /// ```
    pub async fn publish(&self, event_type: &str, event_data: &Event<T>) -> Result<(), BasuError> {
        let event_handler_map = self.event_handler_map.lock().await;

        match event_handler_map.get(event_type) {
            Some(handler_map) => {
                let handler_map = handler_map.lock().await;
                let futures = handler_map.iter().map(|(_id, h)| h.handle(event_data));
                futures::future::try_join_all(futures)
                    .await
                    .map(|_| ())
                    .map_err(Into::into)
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
    /// #[async_trait]
    /// impl Handle<MyEventData> for MyEventHandler {
    ///     async fn handle(&self, event: &Event<MyEventData>) -> Result<(), BasuError> {
    ///         // Handle the event here
    ///         // ...
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let event_bus = EventBus::<MyEventData>::new();
    /// let handler = MyEventHandler;
    /// let _handler_id = event_bus.subscribe("my_event", Box::new(handler)).await;
    ///
    /// let event_types = event_bus.list().await;
    /// for event_type in event_types {
    ///     println!("Registered event type: {}", event_type);
    /// }
    ///```
    pub async fn list(&self) -> Vec<String> {
        let event_handler_map = self.event_handler_map.lock().await;

        event_handler_map.keys().cloned().collect()
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
    /// let handler_count = event_bus.get_handler_count(event_type).await?;
    ///
    /// println!("Number of handlers for event '{}': {}", event_type, handler_count);
    /// ```
    pub async fn get_handler_count(&self, event_type: &str) -> Result<usize, BasuError> {
        let event_handler_map = self.event_handler_map.lock().await;

        match event_handler_map.get(event_type) {
            Some(handler_map) => {
                let handler_map = handler_map.lock().await;
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
    /// #[async_trait]
    /// impl Handle<MyEventData> for MyEventHandler {
    ///     async fn handle(&self, event: &Event<MyEventData>) -> Result<(), BasuError> {
    ///         // Handle the event here
    ///         // ...
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let event_bus = EventBus::<MyEventData>::new();
    /// let handler = MyEventHandler;
    /// let _handler_id = event_bus.subscribe("my_event", Box::new(handler)).await;
    ///
    /// event_bus.clear().await;
    ///
    /// println!("All event handlers cleared");
    /// ```
    ///
    /// **Note:** The `clear` method removes all event handlers and makes the event bus empty.
    pub async fn clear(&self) {
        let mut event_handler_map = self.event_handler_map.lock().await;

        event_handler_map.clear();
    }
}
