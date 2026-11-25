use async_trait::async_trait;

use crate::{
    errors::error::Error,
    ports::{command_port::CommandPort, event_port::EventPort, query_port::QueryPort},
};

#[async_trait]
pub trait ServiceBusPort: Send + Sync {
    async fn command<Command: CommandPort>(
        &self,
        command: Command,
    ) -> Result<Command::Output, Error>;

    async fn query<Query: QueryPort>(&self, query: Query) -> Result<Query::Output, Error>;

    fn event<Event: EventPort>(&self, event: Event);
}
