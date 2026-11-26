# Implementation of CQRS pattern in Rust

Commands, queries and events are plain data. Handlers are separate types that own
their dependencies. Buses connect them: a sender knows only the bus and the message,
a handler knows only the message and its own dependencies.

| Message | Handlers | `send` |
|---|---|---|
| Command | exactly one | `async`, returns `Result<Output, Error>` |
| Query | exactly one | `async`, returns `Result<Output, Error>` |
| Event | exactly one | sync, returns nothing |

## Command

`Output` is the result of a successful command, `handle` wraps it into `Result`.

```rust
use std::sync::Arc;

use async_trait::async_trait;
use kti_cqrs_rs::{
    errors::error::Error,
    ports::{command_port::CommandPort, handler::command_handler_port::CommandHandlerPort},
};

pub struct AddElementCommand {
    pub element: i32,
}

impl CommandPort for AddElementCommand {
    type Output = ();
}

pub struct AddElementHandler {
    repository: Arc<dyn ElementRepositoryCommandPort>,
}

impl AddElementHandler {
    pub fn new(repository: Arc<dyn ElementRepositoryCommandPort>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl CommandHandlerPort<AddElementCommand> for AddElementHandler {
    async fn handle(&self, command: AddElementCommand) -> Result<(), Error> {
        self.repository.add_element(command.element).await
    }
}
```

## Query

```rust
use kti_cqrs_rs::ports::{handler::query_handler_port::QueryHandlerPort, query_port::QueryPort};

pub struct GetCountElementsQuery;

impl QueryPort for GetCountElementsQuery {
    type Output = usize;
}

pub struct GetCountElementsHandler {
    repository: Arc<dyn ElementRepositoryQueryPort>,
}

impl GetCountElementsHandler {
    pub fn new(repository: Arc<dyn ElementRepositoryQueryPort>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl QueryHandlerPort<GetCountElementsQuery> for GetCountElementsHandler {
    async fn handle(&self, _query: GetCountElementsQuery) -> Result<usize, Error> {
        self.repository.get_count().await
    }
}
```

## Event

```rust
use kti_cqrs_rs::ports::{event_port::EventPort, handler::event_handler_port::EventHandlerPort};

pub struct IncrementElementEvent {
    pub element: i32,
}

impl EventPort for IncrementElementEvent {}

pub struct IncrementElementHandler {
    repository: Arc<dyn ElementRepositoryCommandPort>,
}

impl IncrementElementHandler {
    pub fn new(repository: Arc<dyn ElementRepositoryCommandPort>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl EventHandlerPort<IncrementElementEvent> for IncrementElementHandler {
    async fn handle(&self, event: IncrementElementEvent) -> Result<(), Error> {
        self.repository
            .update_element(event.element, event.element + 1)
            .await
    }
}
```

Events are fire-and-forget:

* the handler runs in its own `tokio` task, so a registered event has to be sent inside
  a `tokio` runtime, otherwise `send` panics;
* the sender does not wait for the handler and does not receive its error;
* the order of different events is not guaranteed;
* tasks that are not finished when the runtime stops are lost.

## Buses

Handlers are registered in a builder, `build` checks them and returns a bus. A bus is
immutable after that.

```rust
use kti_cqrs_rs::adapters::{
    command_bus_adapter::CommandBusAdapter, event_bus_adapter::EventBusAdapter,
    query_bus_adapter::QueryBusAdapter,
};
use kti_cqrs_rs::ports::bus::{
    command_bus_port::CommandBusPort, event_bus_port::EventBusPort, query_bus_port::QueryBusPort,
};

async fn run(
    command_repository: Arc<dyn ElementRepositoryCommandPort>,
    query_repository: Arc<dyn ElementRepositoryQueryPort>,
) -> Result<usize, Error> {
    let command_bus = CommandBusAdapter::builder()
        .register(AddElementHandler::new(command_repository.clone()))
        .build()?;

    let query_bus = QueryBusAdapter::builder()
        .register(GetCountElementsHandler::new(query_repository))
        .build()?;

    let event_bus = EventBusAdapter::builder()
        .register(IncrementElementHandler::new(command_repository))
        .build()?;

    command_bus.send(AddElementCommand { element: 1 }).await?;

    event_bus.send(IncrementElementEvent { element: 1 });

    query_bus.send(GetCountElementsQuery).await
}
```

* A second handler for the same message is an error of `build`, it names every such
  message.
* A command or a query without a handler is an error of `send`.
* An event without a handler and an error of an event handler are only logged.

`send` is generic, so the bus ports are not dyn compatible: keep the adapter itself,
for example `Arc<CommandBusAdapter>`.

`ServiceBusPort` describes a single entry point for all three buses: `command`, `query`
and `event`.

## Composing handlers

Cross-cutting concerns are added by wrapping a handler rather than by hooking into the
bus. A wrapper is free to require whatever bounds it needs — here `Clone` on the
command, in order to run the inner handler more than once.

```rust
pub struct Retrying<H> {
    pub inner: H,
    pub attempts: u32,
}

#[async_trait]
impl<Command, H> CommandHandlerPort<Command> for Retrying<H>
where
    Command: CommandPort + Clone,
    H: CommandHandlerPort<Command>,
{
    async fn handle(&self, command: Command) -> Result<Command::Output, Error> {
        for _ in 1..self.attempts {
            let output = self.inner.handle(command.clone()).await;

            if output.is_ok() {
                return output;
            }
        }

        self.inner.handle(command).await
    }
}
```

A wrapper is registered in place of the handler, the sender does not notice it. For a
command that implements `Clone`:

```rust
CommandBusAdapter::builder()
    .register(Retrying {
        inner: AddElementHandler::new(repository),
        attempts: 3,
    })
    .build()?;
```

## Example

The complete example with repositories, a service and tests is in `packages/example`.
