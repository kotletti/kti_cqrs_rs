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

pub struct CommandBusAdapter {
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl CommandBusAdapter {
    pub fn builder() -> CommandBusBuilder {
        CommandBusBuilder::default()
    }
}

#[derive(Default)]
pub struct CommandBusBuilder {
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    duplicates: Vec<&'static str>,
}

impl CommandBusBuilder {
    pub fn register<Command, Handler>(mut self, handler: Handler) -> Self
    where
        Command: CommandPort,
        Handler: CommandHandlerPort<Command> + 'static,
    {
        let type_id = TypeId::of::<Command>();
        let type_name = type_name::<Command>();

        let handler: CommandHandler<Command> = Arc::new(handler);

        if self.handlers.insert(type_id, Box::new(handler)).is_some() {
            self.duplicates.push(type_name);
        }

        self
    }

    pub fn build(self) -> Result<CommandBusAdapter, Error> {
        if !self.duplicates.is_empty() {
            return Err(Error::from(format!(
                "Command handlers are registered more than once: {}",
                self.duplicates.join(", ")
            )));
        }

        Ok(CommandBusAdapter {
            handlers: self.handlers,
        })
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
