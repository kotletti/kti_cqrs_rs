use async_trait::async_trait;

use crate::ports::handler::{
    command_handler_port::CommandHandlerPort, event_handler_port::EventHandlerPort,
    query_handler_port::QueryHandlerPort,
};

#[async_trait]
pub trait ServiceBusPort {
    type Context: Send + 'static;

    async fn command<H>(&self, command: H) -> H::Output
    where
        H: CommandHandlerPort<Context = Self::Context> + Send + 'static,
        H::Output: Send + 'static;

    async fn query<H>(&self, query: H) -> H::Output
    where
        H: QueryHandlerPort<Context = Self::Context> + Send + 'static,
        H::Output: Send + 'static;

    fn event<H>(&self, event: H)
    where
        H: EventHandlerPort<Context = Self::Context> + Send + 'static;
}
