use async_trait::async_trait;

use crate::{
  errors::error::Error,
  ports::handler::{
    command_handler_port::CommandHandlerPort, event_handler_port::EventHandlerPort,
    query_handler_port::QueryHandlerPort,
  },
};

#[async_trait]
pub trait ServiceBusPort {
  type Context;

  async fn command<O>(
    &self,
    command: Box<dyn CommandHandlerPort<Context = Self::Context, Output = O>>,
  ) -> Result<O, Error>;

  async fn query<O>(
    &self,
    query: Box<dyn QueryHandlerPort<Context = Self::Context, Output = O>>,
  ) -> Result<O, Error>;

  fn event(&self, event: Box<dyn EventHandlerPort<Context = Self::Context>>);
}
