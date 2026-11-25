use async_trait::async_trait;

use crate::{errors::error::Error, ports::query_port::QueryPort};

#[async_trait]
pub trait QueryBusPort: Send + Sync {
    async fn send<Query: QueryPort>(&self, query: Query) -> Result<Query::Output, Error>;
}
