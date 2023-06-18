use crate::{error::BasuError, event::Event, EventBus, Handle};

#[derive(Debug)]
struct Data {
    message: String,
}

struct HandlerA;
struct HandlerB;

impl Handle<Data> for HandlerA {
    fn handle(&self, event: &Event<Data>) -> Result<(), BasuError> {
        let data = event.get_data();
        println!("HandlerA: {}", data.message);

        Ok(())
    }
}

impl Handle<Data> for HandlerB {
    fn handle(&self, event: &Event<Data>) -> Result<(), BasuError> {
        let data = event.get_data();
        println!("HandlerB: {}", data.message);

        Ok(())
    }
}

const ECHO: &str = "echo";

#[test]
fn test() {
    let eventbus = EventBus::new();

    let handler_a_id = eventbus.subscribe(ECHO, Box::new(HandlerA)).unwrap();
    println!("HandlerA id: {:?}", handler_a_id);

    let event = Event::new(Data {
        message: "{data from event}".to_owned(),
    });

    eventbus.publish(ECHO, &event).unwrap();
    let event_types = eventbus.list().unwrap();
    assert_eq!(event_types, vec![ECHO.to_owned()]);

    let count = eventbus.get_handler_count(ECHO).unwrap();
    assert_eq!(count, 1);

    let handler_b_id = eventbus.subscribe(ECHO, Box::new(HandlerB)).unwrap();
    println!("HandlerB id: {:?}", handler_b_id);

    let count = eventbus.get_handler_count(ECHO).unwrap();
    assert_eq!(count, 2);
    eventbus.publish(ECHO, &event).unwrap();

    eventbus.unsubscribe(ECHO, &handler_a_id).unwrap();
    let count = eventbus.get_handler_count(ECHO).unwrap();
    assert_eq!(count, 1);

    eventbus.clear().unwrap();
    let event_types = eventbus.list().unwrap();
    assert_eq!(event_types.len(), 0);
}
