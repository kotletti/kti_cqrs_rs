use std::sync::Arc;

use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ElementRepositoryStore {
  store: Arc<RwLock<Vec<i32>>>,
}

impl ElementRepositoryStore {
  pub fn new(store: Arc<RwLock<Vec<i32>>>) -> Self {
    Self { store }
  }

  pub fn get_store(&self) -> Arc<RwLock<Vec<i32>>> {
    self.store.clone()
  }
}
