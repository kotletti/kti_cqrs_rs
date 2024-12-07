use async_trait::async_trait;

use crate::{
  errors::error::Error,
  ports::{
    bus::command_bus_port::CommandBusPort, handler::command_handler_port::CommandHandlerPort,
  },
};

#[derive(Clone, Copy)]
pub struct CommandBusAdapter;

#[async_trait]
impl CommandBusPort for CommandBusAdapter {
  async fn send<C: Send, O>(
    &self,
    command: Box<dyn CommandHandlerPort<Context = C, Output = O>>,
    context: C,
  ) -> Result<O, Error> {
    command.execute(context).await
  }
}
