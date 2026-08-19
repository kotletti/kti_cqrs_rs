use async_trait::async_trait;

use crate::ports::handler::query_handler_port::QueryHandlerPort;

#[async_trait]
pub trait QueryBusPort: Send + Sync {
    async fn send<H>(&self, query: H, context: H::Context) -> H::Output
    where
        H: QueryHandlerPort + Send + 'static,
        H::Context: Send + 'static,
        H::Output: Send + 'static;
}
