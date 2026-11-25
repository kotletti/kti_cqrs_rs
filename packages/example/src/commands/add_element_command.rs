use std::sync::Arc;

use async_trait::async_trait;
use kti_cqrs_rs::{
    errors::error::Error,
    ports::{command_port::CommandPort, handler::command_handler_port::CommandHandlerPort},
};

use crate::ports::element_repository_command_port::ElementRepositoryCommandPort;

pub struct AddElementCommand {
    pub element: i32,
}

impl CommandPort for AddElementCommand {
    type Output = ();
}

pub struct AddElementHandler {
    repository: Arc<dyn ElementRepositoryCommandPort>,
}

impl AddElementHandler {
    pub fn new(repository: Arc<dyn ElementRepositoryCommandPort>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl CommandHandlerPort<AddElementCommand> for AddElementHandler {
    async fn handle(&self, command: AddElementCommand) -> Result<(), Error> {
        self.repository.add_element(command.element).await
    }
}
