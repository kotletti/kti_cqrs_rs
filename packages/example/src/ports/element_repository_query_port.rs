use async_trait::async_trait;
use kti_cqrs_rs::errors::error::Error;

#[async_trait]
pub trait ElementRepositoryQueryPort: Send + Sync {
  async fn get_elements(&self) -> Result<Vec<i32>, Error>;
  async fn get_count(&self) -> Result<usize, Error>;
}
