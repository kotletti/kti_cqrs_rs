use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::services::complex::ports::complex_repository_port::{
  ComplexRepositoryCommandPort, ComplexRepositoryQueryPort,
};

pub struct ComplexRepositoryAdapter {
  store: Arc<Mutex<Vec<i32>>>,
}

impl ComplexRepositoryAdapter {
  pub fn new(store: Arc<Mutex<Vec<i32>>>) -> Self {
    Self { store }
  }
}

#[async_trait]
impl ComplexRepositoryQueryPort for ComplexRepositoryAdapter {
  async fn get_elements(&self) -> Result<Vec<i32>, Box<dyn Error>> {
    Ok((*self.store).lock().await.clone())
  }

  async fn get_count(&self) -> Result<i32, Box<dyn Error>> {
    Ok(self.store.lock().await.len() as i32)
  }
}

#[async_trait]
impl ComplexRepositoryCommandPort for ComplexRepositoryAdapter {
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
}
