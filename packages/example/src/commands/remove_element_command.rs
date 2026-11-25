use std::sync::Arc;

use async_trait::async_trait;
use kti_cqrs_rs::{
    errors::error::Error,
    ports::{command_port::CommandPort, handler::command_handler_port::CommandHandlerPort},
};

use crate::ports::element_repository_command_port::ElementRepositoryCommandPort;

pub struct RemoveElementCommand {
    pub element: i32,
}

impl CommandPort for RemoveElementCommand {
    type Output = ();
}

pub struct RemoveElementHandler {
    repository: Arc<dyn ElementRepositoryCommandPort>,
}

impl RemoveElementHandler {
    pub fn new(repository: Arc<dyn ElementRepositoryCommandPort>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl CommandHandlerPort<RemoveElementCommand> for RemoveElementHandler {
    async fn handle(&self, command: RemoveElementCommand) -> Result<(), Error> {
        self.repository.remove_element(command.element).await
    }
}
