use std::error::Error;

use async_trait::async_trait;

#[async_trait]
pub trait RwLockRepositoryQueryPort: Send + Sync {
  async fn get_elements(&self) -> Result<Vec<i32>, Box<dyn Error>>;
  async fn get_count(&self) -> Result<i32, Box<dyn Error>>;
}

#[async_trait]
pub trait RwLockRepositoryCommandPort: Send + Sync {
  async fn add_element(&self, element: i32) -> Result<(), Box<dyn Error>>;
  async fn remove_element(&self, element: i32) -> Result<(), Box<dyn Error>>;
}
