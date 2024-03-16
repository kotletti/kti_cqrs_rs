use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::services::rw_lock::ports::rw_lock_repository_port::{
  RwLockRepositoryCommandPort, RwLockRepositoryQueryPort,
};

pub struct RwLockRepositoryAdapter {
  store: Arc<RwLock<Vec<i32>>>,
}

impl RwLockRepositoryAdapter {
  pub fn new(store: Arc<RwLock<Vec<i32>>>) -> Self {
    Self { store }
  }
}

#[async_trait]
impl RwLockRepositoryQueryPort for RwLockRepositoryAdapter {
  async fn get_elements(&self) -> Result<Vec<i32>, Box<dyn Error>> {
    let store = self.store.read().await;

    Ok(store.clone())
  }

  async fn get_count(&self) -> Result<i32, Box<dyn Error>> {
    let store = self.store.read().await;

    let count: i32 = store.len().try_into().unwrap();

    Ok(count)
  }
}

#[async_trait]
impl RwLockRepositoryCommandPort for RwLockRepositoryAdapter {
  async fn add_element(&self, element: i32) -> Result<(), Box<dyn Error>> {
    let mut store = self.store.write().await;

    let has_element = store.iter().any(|x| *x == element);

    if has_element {
      return Err(format!("Element {} already exists", element).into());
    }

    store.push(element);

    Ok(())
  }

  async fn remove_element(&self, element: i32) -> Result<(), Box<dyn Error>> {
    let mut store = self.store.write().await;

    let index = store.iter().position(|x| *x == element);

    match index {
      Some(i) => {
        store.remove(i);
        Ok(())
      }
      None => Err(format!("Element {} does not exist", element).into()),
    }
  }
}
