use async_trait::async_trait;

use crate::{errors::error::Error, ports::command_port::CommandPort};

#[async_trait]
pub trait CommandBusPort: Send + Sync {
    async fn send<Command: CommandPort>(&self, command: Command) -> Result<Command::Output, Error>;
}
