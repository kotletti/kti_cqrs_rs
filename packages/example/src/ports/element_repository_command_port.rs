use async_trait::async_trait;
use kti_cqrs_rs::errors::error::Error;

#[async_trait]
pub trait ElementRepositoryCommandPort: Send + Sync {
  async fn add_element(&self, element: i32) -> Result<(), Error>;
  async fn remove_element(&self, element: i32) -> Result<(), Error>;
  async fn update_element(&self, from_element: i32, to_element: i32) -> Result<(), Error>;
}
