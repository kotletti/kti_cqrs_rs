# Implementation of CQRS pattern in Rust

### Currently the crate contains only query, command, events handlers

## Handlers

A handler carries both the message and its execution. The crate does not impose an
error type — `Output` is yours, so put a `Result` into it when you need one.

### Command

```rust
use std::sync::Arc;

use async_trait::async_trait;
use kti_cqrs_rs::errors::error::Error;
use kti_cqrs_rs::ports::handler::command_handler_port::CommandHandlerPort;
use tokio::sync::RwLock;

pub struct AddElementCommand {
  pub element: i32,
}

#[async_trait]
impl CommandHandlerPort for AddElementCommand {
  type Context = Arc<RwLock<AppContext>>;
  type Output = Result<(), Error>;

  async fn execute(&self, context: Self::Context) -> Self::Output {
    let ctx = context.read().await;

    let command_repository = ctx.get_command_repository();
    command_repository.add_element(self.element).await
  }
}
```

### Query

```rust
#[async_trait]
impl QueryHandlerPort for GetElementsQuery {
  type Context = Arc<RwLock<AppContext>>;
  type Output = Result<Vec<i32>, Error>;

  async fn execute(&self, context: Self::Context) -> Self::Output {
    let ctx = context.read().await;

    ctx.get_query_repository().get_elements().await
  }
}
```

### Event

`EventBusAdapter` executes events synchronously via `await`, so errors can be
handled in the adapter. The handler returns `Result<(), Error>` and the adapter
logs errors or handles them as needed.

```rust
#[async_trait]
impl EventHandlerPort for IncrementElementEvent {
  type Context = Arc<RwLock<AppContext>>;

  async fn execute(&self, context: Self::Context) -> Result<(), Error> {
    let ctx = context.read().await;

    let command_repository = ctx.get_command_repository();
    command_repository
      .update_element(self.element, self.element + 1)
      .await
  }
}
```

## Composing handlers

Handlers are plain generic types, so cross-cutting concerns are added by wrapping a
handler rather than by hooking into the bus. A wrapper is free to require whatever
bounds it needs — here `Clone` on the context, in order to run the inner handler
more than once.

```rust
pub struct Retrying<H> {
  pub inner: H,
  pub attempts: u32,
}

#[async_trait]
impl<H, T, E> CommandHandlerPort for Retrying<H>
where
  H: CommandHandlerPort<Output = Result<T, E>>,
  H::Context: Clone + Send + 'static,
  T: Send + 'static,
  E: Send + 'static,
{
  type Context = H::Context;
  type Output = H::Output;

  async fn execute(&self, context: Self::Context) -> Self::Output {
    for _ in 1..self.attempts {
      let output = self.inner.execute(context.clone()).await;

      if output.is_ok() {
        return output;
      }
    }

    self.inner.execute(context).await
  }
}
```

Wrappers compose with each other, and the concrete handler type is still known inside
them, so `std::any::type_name::<H>()` yields the real handler name for logging or
tracing. Nothing is erased unless you box a handler yourself — `Box<H>` is a handler
too, so both forms go through the same call.

```rust
let command = Retrying {
  inner: AddElementCommand { element: 1 },
  attempts: 3,
};

CommandBusAdapter.send(command, context).await
```

## Simple example (existed in repo)

```rust
#[cfg(test)]
mod tests {
  use std::{sync::Arc, time::Duration};

  use kti_cqrs_rs::adapters::{
    command_bus_adapter::CommandBusAdapter, event_bus_adapter::EventBusAdapter,
    query_bus_adapter::QueryBusAdapter,
  };
  use tokio::{sync::RwLock, time::sleep};

  use crate::{
    adapters::{
      element_repository_command_adapter::ElementRepositoryCommandAdapter,
      element_repository_query_adapter::ElementRepositoryQueryAdapter,
      element_service_adapter::ElementServiceAdapter,
    },
    contexts::app_context::AppContext,
    ports::element_service_port::ElementServicePort,
    stores::element_repository_store::ElementRepositoryStore,
  };

  fn create_service() -> Box<dyn ElementServicePort> {
    let store = ElementRepositoryStore::new(Arc::new(RwLock::new(vec![])));

    let query_repository = Box::new(ElementRepositoryQueryAdapter::new(store.clone()));
    let command_repository = Box::new(ElementRepositoryCommandAdapter::new(store.clone()));

    Box::new(ElementServiceAdapter::new(
      Arc::new(RwLock::new(AppContext::new(
        query_repository,
        command_repository,
      ))),
      CommandBusAdapter,
      QueryBusAdapter,
      EventBusAdapter,
    ))
  }

  #[tokio::test]
  async fn should_be_empty_vector() {
    let service = create_service();

    let count = service.get_count().await.unwrap();

    assert_eq!(count, 0);
  }

  #[tokio::test]
  async fn should_be_one_element() {
    let service = create_service();

    service.add_element(1).await.unwrap();

    let count = service.get_count().await.unwrap();

    assert_eq!(count, 1);
  }

  #[tokio::test]
  async fn should_be_empty_after_remove() {
    let service = create_service();

    service.add_element(1).await.unwrap();
    service.remove_element(1).await.unwrap();

    let count = service.get_count().await.unwrap();

    assert_eq!(count, 0);
  }

  #[tokio::test]
  async fn should_be_error_on_remove_not_existed_element() {
    let service = create_service();

    let res = service.remove_element(1).await;

    assert!(res.is_err());
  }

  #[tokio::test]
  async fn should_be_updated_element() {
    let from_element = 1;
    let to_element = 2;
    let service = create_service();

    service.add_element(from_element).await.unwrap();
    service
      .update_element(from_element, to_element)
      .await
      .unwrap();

    let elements = service.get_elements().await.unwrap();

    let updated_element = elements.first().unwrap();

    assert_eq!(*updated_element, to_element);
  }

  #[tokio::test]
  async fn should_be_incremented_by_event() {
    let element = 43;
    let service = create_service();

    service.add_element_with_event(element).await.unwrap();

    sleep(Duration::from_secs(1)).await;

    let elements = service.get_elements().await.unwrap();

    let incremented_element = elements.first().unwrap();

    assert_eq!(*incremented_element, element + 1);
  }

  #[tokio::test]
  async fn should_be_failed_incremented_by_event() {
    let element = 42;
    let service = create_service();

    service.add_element_with_event(element).await.unwrap();

    sleep(Duration::from_secs(1)).await;

    let elements = service.get_elements().await.unwrap();

    let incremented_element = elements.first().unwrap();

    assert_eq!(*incremented_element, element);
  }
}
```
