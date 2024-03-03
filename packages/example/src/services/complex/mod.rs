pub mod adapters;
pub mod commands;
pub mod contexts;
pub mod ports;
pub mod queries;

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use kti_cqrs_rs::core::bus::{CommandBus, QueryBus};
  use tokio::sync::Mutex;

  use super::{
    adapters::{
      complex_repository_adapter::ComplexRepositoryAdapter,
      complex_service_adapter::ComplexServiceAdapter,
    },
    contexts::complex_context::ComplexContext,
    ports::complex_service_port::ComplexServicePort,
  };

  fn create_service() -> Box<dyn ComplexServicePort> {
    let store = Arc::new(Mutex::new(vec![]));

    let query_repository = Box::new(ComplexRepositoryAdapter::new(store.clone()));
    let command_repository = Box::new(ComplexRepositoryAdapter::new(store));

    Box::new(ComplexServiceAdapter::new(
      Arc::new(Mutex::new(ComplexContext::new(
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
  async fn should_be_empty_vector_after_remove() {
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
