use async_trait::async_trait;

use crate::ports::handler::event_handler_port::EventHandlerPort;

#[async_trait]
pub trait EventBusPort: Send + Sync {
    async fn send<H>(&self, event: H, context: H::Context)
    where
        H: EventHandlerPort + Send + 'static,
        H::Context: Send + 'static;
}
