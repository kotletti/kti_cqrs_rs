use std::sync::Arc;

use async_trait::async_trait;

use crate::{errors::error::Error, ports::command_port::CommandPort};

#[async_trait]
pub trait CommandHandlerPort<Command: CommandPort>: Send + Sync {
    async fn handle(&self, command: Command) -> Result<Command::Output, Error>;
}

pub type CommandHandler<Command> = Arc<dyn CommandHandlerPort<Command>>;
