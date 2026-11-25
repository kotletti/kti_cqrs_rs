use std::sync::Arc;

use async_trait::async_trait;
use kti_cqrs_rs::{
    errors::error::Error,
    ports::{command_port::CommandPort, handler::command_handler_port::CommandHandlerPort},
};

use crate::ports::element_repository_command_port::ElementRepositoryCommandPort;

pub struct UpdateElementCommand {
    pub from_element: i32,
    pub to_element: i32,
}

impl CommandPort for UpdateElementCommand {
    type Output = ();
}

pub struct UpdateElementHandler {
    repository: Arc<dyn ElementRepositoryCommandPort>,
}

impl UpdateElementHandler {
    pub fn new(repository: Arc<dyn ElementRepositoryCommandPort>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl CommandHandlerPort<UpdateElementCommand> for UpdateElementHandler {
    async fn handle(&self, command: UpdateElementCommand) -> Result<(), Error> {
        self.repository
            .update_element(command.from_element, command.to_element)
            .await
    }
}
