use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

#[async_trait]
pub trait CommandHandler: Send + Sync {
  type Context;
  type Output;

  async fn execute(&self, context: Arc<Mutex<Self::Context>>) -> Self::Output;
}
