use crate::{async_trait, error::BasuError, event::Event, EventBus, Handle};

#[derive(Debug)]
struct Data {
    message: String,
}

struct HandlerA;
struct HandlerB;

#[async_trait]
impl Handle<Data> for HandlerA {
    async fn handle(&self, event: &Event<Data>) -> Result<(), BasuError> {
        let data = event.get_data();
        println!("HandlerA: {}", data.message);

        Ok(())
    }
}

#[async_trait]
impl Handle<Data> for HandlerB {
    async fn handle(&self, event: &Event<Data>) -> Result<(), BasuError> {
        let data = event.get_data();
        println!("HandlerB: {}", data.message);

        Ok(())
    }
}

const ECHO: &str = "echo";

#[tokio::test]
async fn test() {
    let eventbus = EventBus::new();

    let handler_a_id = eventbus.subscribe(ECHO, Box::new(HandlerA)).await;
    println!("HandlerA id: {:?}", handler_a_id);

    let event = Event::new(Data {
        message: "{data from event}".to_owned(),
    });

    eventbus.publish(ECHO, &event).await.unwrap();
    let event_types = eventbus.list().await;
    assert_eq!(event_types, vec![ECHO.to_owned()]);

    let count = eventbus.get_handler_count(ECHO).await.unwrap();
    assert_eq!(count, 1);

    let handler_b_id = eventbus.subscribe(ECHO, Box::new(HandlerB)).await;
    println!("HandlerB id: {:?}", handler_b_id);

    let count = eventbus.get_handler_count(ECHO).await.unwrap();
    assert_eq!(count, 2);
    eventbus.publish(ECHO, &event).await.unwrap();

    eventbus.unsubscribe(ECHO, &handler_a_id).await.unwrap();
    let count = eventbus.get_handler_count(ECHO).await.unwrap();
    assert_eq!(count, 1);

    eventbus.clear().await;
    let event_types = eventbus.list().await;
    assert_eq!(event_types.len(), 0);
}
