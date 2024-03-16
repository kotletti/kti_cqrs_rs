use async_trait::async_trait;

use crate::common::handler::{command_handler::CommandHandler, query_handler::QueryHandler};

#[async_trait]
pub trait ServiceBus {
  type Context;

  async fn command<O>(
    &self,
    command: Box<dyn CommandHandler<Context = Self::Context, Output = O>>,
  ) -> O;

  async fn query<O>(&self, query: Box<dyn QueryHandler<Context = Self::Context, Output = O>>) -> O;
}
