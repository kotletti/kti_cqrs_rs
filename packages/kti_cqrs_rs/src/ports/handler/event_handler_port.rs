use async_trait::async_trait;

use crate::errors::error::Error;

#[async_trait]
pub trait EventHandlerPort: Send + Sync {
  type Context;

  async fn execute(&self, context: Self::Context) -> Result<(), Error>;
}
