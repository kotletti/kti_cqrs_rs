use std::sync::Arc;

use async_trait::async_trait;

use crate::{errors::error::Error, ports::query_port::QueryPort};

#[async_trait]
pub trait QueryHandlerPort<Query: QueryPort>: Send + Sync {
    async fn handle(&self, query: Query) -> Result<Query::Output, Error>;
}

pub type QueryHandler<Query> = Arc<dyn QueryHandlerPort<Query>>;
