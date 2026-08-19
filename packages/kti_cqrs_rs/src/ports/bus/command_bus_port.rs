use async_trait::async_trait;

use crate::ports::handler::command_handler_port::CommandHandlerPort;

#[async_trait]
pub trait CommandBusPort: Send + Sync {
    async fn send<H>(&self, command: H, context: H::Context) -> H::Output
    where
        H: CommandHandlerPort + Send + 'static,
        H::Context: Send + 'static,
        H::Output: Send + 'static;
}
