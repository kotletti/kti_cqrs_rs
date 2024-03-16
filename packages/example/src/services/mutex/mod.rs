pub mod adapters;
pub mod commands;
pub mod contexts;
pub mod ports;
pub mod queries;

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use kti_cqrs_rs::core::bus::{command_bus::CommandBus, query_bus::QueryBus};
  use tokio::sync::Mutex;

  use super::{
    adapters::{
      mutex_repository_adapter::MutexRepositoryAdapter, mutex_service_adapter::MutexServiceAdapter,
    },
    contexts::mutex_context::MutexContext,
    ports::mutex_service_port::MutexServicePort,
  };

  fn create_service() -> Box<dyn MutexServicePort> {
    let store = Arc::new(Mutex::new(vec![]));

    let query_repository = Box::new(MutexRepositoryAdapter::new(store.clone()));
    let command_repository = Box::new(MutexRepositoryAdapter::new(store));

    Box::new(MutexServiceAdapter::new(
      Arc::new(Mutex::new(MutexContext::new(
        query_repository,
        command_repository,
      ))),
      CommandBus,
      QueryBus,
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
}
