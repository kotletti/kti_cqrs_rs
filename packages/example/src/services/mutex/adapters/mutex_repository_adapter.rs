use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::services::mutex::ports::mutex_repository_port::{
  MutexRepositoryCommandPort, MutexRepositoryQueryPort,
};

pub struct MutexRepositoryAdapter {
  store: Arc<Mutex<Vec<i32>>>,
}

impl MutexRepositoryAdapter {
  pub fn new(store: Arc<Mutex<Vec<i32>>>) -> Self {
    Self { store }
  }
}

#[async_trait]
impl MutexRepositoryQueryPort for MutexRepositoryAdapter {
  async fn get_elements(&self) -> Result<Vec<i32>, Box<dyn Error>> {
    Ok((*self.store).lock().await.clone())
  }

  async fn get_count(&self) -> Result<i32, Box<dyn Error>> {
    Ok(self.store.lock().await.len() as i32)
  }
}

#[async_trait]
impl MutexRepositoryCommandPort for MutexRepositoryAdapter {
  async fn add_element(&self, element: i32) -> Result<(), Box<dyn Error>> {
    let mut store = self.store.lock().await;

    let has_element = store.iter().any(|x| *x == element);

    if has_element {
      return Err(format!("Element {} already exists", element).into());
    }

    store.push(element);

    Ok(())
  }

  async fn remove_element(&self, element: i32) -> Result<(), Box<dyn Error>> {
    let mut store = self.store.lock().await;

    let index = store.iter().position(|x| *x == element);

    match index {
      Some(i) => {
        store.remove(i);
        Ok(())
      }
      None => Err(format!("Element {} does not exist", element).into()),
    }
  }

  async fn update_element(&self, from_element: i32, to_element: i32) -> Result<(), Box<dyn Error>> {
    let mut store = self.store.lock().await;

    let index = store.iter().position(|x| *x == from_element);

    match index {
      Some(i) => {
        store[i] = to_element;
        Ok(())
      }
      None => Err(format!("Element {} does not exist", from_element).into()),
    }
  }
}
