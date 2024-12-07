use async_trait::async_trait;
use kti_cqrs_rs::errors::error::Error;

use crate::{
  ports::element_repository_command_port::ElementRepositoryCommandPort,
  stores::element_repository_store::ElementRepositoryStore,
};

pub struct ElementRepositoryCommandAdapter {
  store: ElementRepositoryStore,
}

impl ElementRepositoryCommandAdapter {
  pub fn new(store: ElementRepositoryStore) -> Self {
    Self { store }
  }
}

#[async_trait]
impl ElementRepositoryCommandPort for ElementRepositoryCommandAdapter {
  async fn add_element(&self, element: i32) -> Result<(), Error> {
    let store = self.store.get_store();
    let mut store = store.write().await;

    let has_element = store.iter().any(|x| *x == element);

    if has_element {
      return Err(format!("Element {} already exists", element).into());
    }

    store.push(element);

    Ok(())
  }

  async fn remove_element(&self, element: i32) -> Result<(), Error> {
    let store = self.store.get_store();
    let mut store = store.write().await;

    let index = store.iter().position(|x| *x == element);

    match index {
      Some(i) => {
        store.remove(i);
        Ok(())
      }
      None => Err(format!("Element {} does not exist", element).into()),
    }
  }

  async fn update_element(&self, from_element: i32, to_element: i32) -> Result<(), Error> {
    let store = self.store.get_store();
    let mut store = store.write().await;

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
