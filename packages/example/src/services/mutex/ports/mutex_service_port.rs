use std::error::Error;

use async_trait::async_trait;

#[async_trait]
pub trait MutexServicePort {
  async fn get_elements(&self) -> Result<Vec<i32>, Box<dyn Error>>;
  async fn add_element(&self, element: i32) -> Result<(), Box<dyn Error>>;
  async fn add_element_with_event(&self, element: i32) -> Result<(), Box<dyn Error>>;
  async fn update_element(&self, from_element: i32, to_element: i32) -> Result<(), Box<dyn Error>>;
  async fn remove_element(&self, element: i32) -> Result<(), Box<dyn Error>>;
  async fn get_count(&self) -> Result<i32, Box<dyn Error>>;
}
