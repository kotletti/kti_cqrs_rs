use async_trait::async_trait;

#[async_trait]
pub trait QueryHandler: Send + Sync {
  type Context;
  type Output;

  async fn execute(&self, context: Self::Context) -> Self::Output;
}
