use async_trait::async_trait;

use crate::errors::error::Error;

#[async_trait]
pub trait QueryHandlerPort: Send + Sync {
  type Context;
  type Output;

  async fn execute(&self, context: Self::Context) -> Result<Self::Output, Error>;
}
