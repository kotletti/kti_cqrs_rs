use async_trait::async_trait;
use kti_cqrs_rs::errors::error::Error;

#[async_trait]
pub trait ElementServicePort {
  async fn get_elements(&self) -> Result<Vec<i32>, Error>;
  async fn add_element(&self, element: i32) -> Result<(), Error>;
  async fn add_element_with_event(&self, element: i32) -> Result<(), Error>;
  async fn update_element(&self, from_element: i32, to_element: i32) -> Result<(), Error>;
  async fn remove_element(&self, element: i32) -> Result<(), Error>;
  async fn get_count(&self) -> Result<usize, Error>;
}
