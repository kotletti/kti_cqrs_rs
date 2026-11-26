use std::time::Duration;

use async_trait::async_trait;
use tokio::{
    sync::mpsc::{UnboundedSender, unbounded_channel},
    time::timeout,
};

use crate::{
    adapters::{
        command_bus_adapter::CommandBusAdapter, event_bus_adapter::EventBusAdapter,
        query_bus_adapter::QueryBusAdapter,
    },
    errors::error::Error,
    ports::{
        bus::{
            command_bus_port::CommandBusPort, event_bus_port::EventBusPort,
            query_bus_port::QueryBusPort, service_bus_port::ServiceBusPort,
        },
        command_port::CommandPort,
        event_port::EventPort,
        handler::{
            command_handler_port::CommandHandlerPort, event_handler_port::EventHandlerPort,
            query_handler_port::QueryHandlerPort,
        },
        query_port::QueryPort,
    },
};

struct ServiceBus {
    command: CommandBusAdapter,
    query: QueryBusAdapter,
    event: EventBusAdapter,
}

#[async_trait]
impl ServiceBusPort for ServiceBus {
    async fn command<Command: CommandPort>(
        &self,
        command: Command,
    ) -> Result<Command::Output, Error> {
        self.command.send(command).await
    }

    async fn query<Query: QueryPort>(&self, query: Query) -> Result<Query::Output, Error> {
        self.query.send(query).await
    }

    fn event<Event: EventPort>(&self, event: Event) {
        self.event.send(event);
    }
}

struct PingCommand;

impl CommandPort for PingCommand {
    type Output = &'static str;
}

struct PingQuery;

impl QueryPort for PingQuery {
    type Output = usize;
}

struct PingEvent;

impl EventPort for PingEvent {}

struct PingHandler(UnboundedSender<()>);

#[async_trait]
impl CommandHandlerPort<PingCommand> for PingHandler {
    async fn handle(&self, _command: PingCommand) -> Result<&'static str, Error> {
        Ok("pong")
    }
}

#[async_trait]
impl QueryHandlerPort<PingQuery> for PingHandler {
    async fn handle(&self, _query: PingQuery) -> Result<usize, Error> {
        Ok(1)
    }
}

#[async_trait]
impl EventHandlerPort<PingEvent> for PingHandler {
    async fn handle(&self, _event: PingEvent) -> Result<(), Error> {
        self.0.send(())?;

        Ok(())
    }
}

#[tokio::test]
async fn should_send_every_message_through_service_bus() -> Result<(), Error> {
    let (sender, mut receiver) = unbounded_channel();

    let bus = ServiceBus {
        command: CommandBusAdapter::builder()
            .register(PingHandler(sender.clone()))
            .build()?,
        query: QueryBusAdapter::builder()
            .register(PingHandler(sender.clone()))
            .build()?,
        event: EventBusAdapter::builder()
            .register(PingHandler(sender))
            .build()?,
    };

    assert_eq!(bus.command(PingCommand).await?, "pong");
    assert_eq!(bus.query(PingQuery).await?, 1);

    bus.event(PingEvent);

    assert_eq!(
        timeout(Duration::from_secs(1), receiver.recv()).await?,
        Some(())
    );

    Ok(())
}
