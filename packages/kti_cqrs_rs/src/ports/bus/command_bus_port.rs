use async_trait::async_trait;

use crate::{errors::error::Error, ports::handler::command_handler_port::CommandHandlerPort};

#[async_trait]
pub trait CommandBusPort: Send + Sync {
  async fn send<C: Send, O>(
    &self,
    command: Box<dyn CommandHandlerPort<Context = C, Output = O>>,
    context: C,
  ) -> Result<O, Error>;
}
