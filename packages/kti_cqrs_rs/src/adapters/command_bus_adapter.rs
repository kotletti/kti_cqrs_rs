use async_trait::async_trait;

use crate::ports::{
    bus::command_bus_port::CommandBusPort, handler::command_handler_port::CommandHandlerPort,
};

#[derive(Clone, Copy)]
pub struct CommandBusAdapter;

#[async_trait]
impl CommandBusPort for CommandBusAdapter {
    async fn send<H>(&self, command: H, context: H::Context) -> H::Output
    where
        H: CommandHandlerPort + Send + 'static,
        H::Context: Send + 'static,
        H::Output: Send + 'static,
    {
        command.execute(context).await
    }
}
