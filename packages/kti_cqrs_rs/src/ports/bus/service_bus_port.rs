use async_trait::async_trait;

use crate::{
  common::handler::{event_handler::EventHandler, query_handler::QueryHandler},
  errors::error::Error,
};

use super::command_handler_port::CommandHandlerPort;

#[async_trait]
pub trait ServiceBusPort {
  type Context;

  async fn command<O>(
    &self,
    command: Box<dyn CommandHandlerPort<Context = Self::Context, Output = O>>,
  ) -> Result<O, Error>;

  async fn query<O>(
    &self,
    query: Box<dyn QueryHandler<Context = Self::Context, Output = O>>,
  ) -> Result<O, Error>;

  fn event(&self, event: Box<dyn EventHandler<Context = Self::Context>>);
}
