/// Abstraction for representing event that can hold any data type.
#[derive(Debug)]
pub struct Event<T> {
    /// event data which can be processed by handler
    pub data: T,
}

impl<T> Event<T> {
    /// create a new event.
    /// ## Example
    ///
    /// ```no_run
    ///struct MyEventData {
    ///    // Define your event data structure here
    /// }
    ///
    /// // Create a new event
    /// let event_data = MyEventData { /* initialize your event data */ };
    /// let event = Event::new(event_data);
    /// ```
    pub fn new(data: T) -> Event<T> {
        Event { data }
    }

    /// return the data that held in event.
    pub fn get_data(&self) -> &T {
        &self.data
    }
}
