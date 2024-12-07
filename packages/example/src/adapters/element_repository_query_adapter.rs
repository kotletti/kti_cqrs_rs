use async_trait::async_trait;
use kti_cqrs_rs::errors::error::Error;

use crate::{
  ports::element_repository_query_port::ElementRepositoryQueryPort,
  stores::element_repository_store::ElementRepositoryStore,
};

pub struct ElementRepositoryQueryAdapter {
  store: ElementRepositoryStore,
}

impl ElementRepositoryQueryAdapter {
  pub fn new(store: ElementRepositoryStore) -> Self {
    Self { store }
  }
}

#[async_trait]
impl ElementRepositoryQueryPort for ElementRepositoryQueryAdapter {
  async fn get_elements(&self) -> Result<Vec<i32>, Error> {
    let store = self.store.get_store();

    let elements = store.read().await.clone();

    Ok(elements)
  }

  async fn get_count(&self) -> Result<usize, Error> {
    let store = self.store.get_store();

    let count = store.read().await.len();

    Ok(count)
  }
}
