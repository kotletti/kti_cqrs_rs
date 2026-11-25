use std::{
    any::{Any, TypeId, type_name},
    collections::HashMap,
    sync::Arc,
};

use async_trait::async_trait;

use crate::{
    errors::error::Error,
    ports::{
        bus::command_bus_port::CommandBusPort,
        command_port::CommandPort,
        handler::command_handler_port::{CommandHandler, CommandHandlerPort},
    },
};

#[derive(Default)]
pub struct CommandBusAdapter {
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl CommandBusAdapter {
    pub fn register<Command, Handler>(mut self, handler: Handler) -> Result<Self, Error>
    where
        Command: CommandPort,
        Handler: CommandHandlerPort<Command> + 'static,
    {
        let type_id = TypeId::of::<Command>();
        let type_name = type_name::<Command>();

        let handler: CommandHandler<Command> = Arc::new(handler);

        if self.handlers.insert(type_id, Box::new(handler)).is_some() {
            return Err(Error::from(format!(
                "Command handler is already registered: {type_name}"
            )));
        }

        Ok(self)
    }
}

#[async_trait]
impl CommandBusPort for CommandBusAdapter {
    async fn send<Command: CommandPort>(&self, command: Command) -> Result<Command::Output, Error> {
        let type_id = TypeId::of::<Command>();
        let type_name = type_name::<Command>();

        let Some(handler) = self
            .handlers
            .get(&type_id)
            .and_then(|h| h.downcast_ref::<CommandHandler<Command>>())
        else {
            return Err(Error::from(format!(
                "Command handler not found: {type_name}"
            )));
        };

        handler.handle(command).await
    }
}
