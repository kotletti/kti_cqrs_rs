use std::error::Error;

use async_trait::async_trait;

#[async_trait]
pub trait EventHandler: Send + Sync {
  type Context;

  async fn execute(&self, context: Self::Context) -> Result<(), Box<dyn Error>>;
}
