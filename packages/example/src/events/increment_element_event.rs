use std::sync::Arc;

use async_trait::async_trait;
use kti_cqrs_rs::{
    errors::error::Error,
    ports::{event_port::EventPort, handler::event_handler_port::EventHandlerPort},
};

use crate::ports::element_repository_command_port::ElementRepositoryCommandPort;

pub struct IncrementElementEvent {
    pub element: i32,
}

impl EventPort for IncrementElementEvent {}

pub struct IncrementElementHandler {
    repository: Arc<dyn ElementRepositoryCommandPort>,
}

impl IncrementElementHandler {
    pub fn new(repository: Arc<dyn ElementRepositoryCommandPort>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl EventHandlerPort<IncrementElementEvent> for IncrementElementHandler {
    async fn handle(&self, event: IncrementElementEvent) -> Result<(), Error> {
        self.repository
            .update_element(event.element, event.element + 1)
            .await
    }
}
