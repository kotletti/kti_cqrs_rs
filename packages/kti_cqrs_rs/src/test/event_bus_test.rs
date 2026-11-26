use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use tokio::{
    sync::{
        Notify,
        mpsc::{UnboundedSender, unbounded_channel},
    },
    time::timeout,
};

use crate::{
    adapters::event_bus_adapter::EventBusAdapter,
    errors::error::Error,
    ports::{
        bus::event_bus_port::EventBusPort, event_port::EventPort,
        handler::event_handler_port::EventHandlerPort,
    },
};

const TIMEOUT: Duration = Duration::from_secs(1);

struct PingEvent(i32);

impl EventPort for PingEvent {}

struct FailEvent;

impl EventPort for FailEvent {}

struct PanicEvent;

impl EventPort for PanicEvent {}

struct PingHandler(UnboundedSender<i32>);

#[async_trait]
impl EventHandlerPort<PingEvent> for PingHandler {
    async fn handle(&self, event: PingEvent) -> Result<(), Error> {
        self.0.send(event.0)?;

        Ok(())
    }
}

struct FailHandler;

#[async_trait]
impl EventHandlerPort<FailEvent> for FailHandler {
    async fn handle(&self, _event: FailEvent) -> Result<(), Error> {
        Err(Error::from("Event failed"))
    }
}

struct PanicHandler;

#[async_trait]
impl EventHandlerPort<PanicEvent> for PanicHandler {
    async fn handle(&self, _event: PanicEvent) -> Result<(), Error> {
        panic!("Event panicked")
    }
}

struct WaitingHandler {
    release: Arc<Notify>,
    sender: UnboundedSender<i32>,
}

#[async_trait]
impl EventHandlerPort<PingEvent> for WaitingHandler {
    async fn handle(&self, event: PingEvent) -> Result<(), Error> {
        self.release.notified().await;
        self.sender.send(event.0)?;

        Ok(())
    }
}

#[tokio::test]
async fn should_deliver_event_to_its_handler() -> Result<(), Error> {
    let (sender, mut receiver) = unbounded_channel();
    let bus = EventBusAdapter::builder()
        .register(FailHandler)
        .register(PingHandler(sender))
        .build()?;

    bus.send(PingEvent(42));

    assert_eq!(timeout(TIMEOUT, receiver.recv()).await?, Some(42));

    Ok(())
}

#[tokio::test]
async fn should_not_wait_for_handler() -> Result<(), Error> {
    let release = Arc::new(Notify::new());
    let (sender, mut receiver) = unbounded_channel();
    let bus = EventBusAdapter::builder()
        .register(WaitingHandler {
            release: release.clone(),
            sender,
        })
        .build()?;

    bus.send(PingEvent(42));

    assert!(receiver.try_recv().is_err());

    release.notify_one();

    assert_eq!(timeout(TIMEOUT, receiver.recv()).await?, Some(42));

    Ok(())
}

#[tokio::test]
async fn should_keep_working_after_handler_error_and_panic() -> Result<(), Error> {
    let (sender, mut receiver) = unbounded_channel();
    let bus = EventBusAdapter::builder()
        .register(FailHandler)
        .register(PanicHandler)
        .register(PingHandler(sender))
        .build()?;

    bus.send(FailEvent);
    bus.send(PanicEvent);
    bus.send(PingEvent(42));

    assert_eq!(timeout(TIMEOUT, receiver.recv()).await?, Some(42));

    Ok(())
}

#[tokio::test]
async fn should_ignore_event_without_handler() -> Result<(), Error> {
    let (sender, mut receiver) = unbounded_channel();
    let bus = EventBusAdapter::builder()
        .register(PingHandler(sender))
        .build()?;

    bus.send(FailEvent);
    bus.send(PingEvent(42));

    assert_eq!(timeout(TIMEOUT, receiver.recv()).await?, Some(42));

    Ok(())
}

#[test]
fn should_error_when_event_handler_is_registered_twice() -> Result<(), Error> {
    let error = EventBusAdapter::builder()
        .register(FailHandler)
        .register(FailHandler)
        .build()
        .err()
        .ok_or("Expected an error")?
        .to_string();

    assert!(
        error.contains("Event handlers are registered more than once"),
        "{error}"
    );
    assert!(error.contains("FailEvent"), "{error}");

    Ok(())
}

#[test]
#[should_panic(expected = "no reactor running")]
fn should_panic_outside_of_runtime() {
    let bus = EventBusAdapter::builder()
        .register(FailHandler)
        .build()
        .unwrap_or_else(|e| panic!("{e}"));

    bus.send(FailEvent);
}

#[test]
fn should_report_every_event_registered_twice() -> Result<(), Error> {
    let error = EventBusAdapter::builder()
        .register(FailHandler)
        .register(PanicHandler)
        .register(FailHandler)
        .register(PanicHandler)
        .build()
        .err()
        .ok_or("Expected an error")?
        .to_string();

    assert!(error.contains("FailEvent"), "{error}");
    assert!(error.contains("PanicEvent"), "{error}");

    Ok(())
}
