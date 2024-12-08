# Implementation of CQRS pattern in Rust

### Currently the crate contains only query, command, events handlers

Simple example (existed in repo)

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

  #[tokio::test]
  async fn should_be_failed_increment_without_awaiting() {
    let element = 43;
    let service = create_service();

    service.add_element_with_event(element).await.unwrap();

    let elements = service.get_elements().await.unwrap();

    let incremented_element = elements.first().unwrap();

    assert_eq!(*incremented_element, element);
  }
}
```
