use std::sync::Arc;

use async_trait::async_trait;
use kti_cqrs_rs::{
    errors::error::Error,
    ports::{handler::query_handler_port::QueryHandlerPort, query_port::QueryPort},
};

use crate::ports::element_repository_query_port::ElementRepositoryQueryPort;

pub struct GetCountElementsQuery;

impl QueryPort for GetCountElementsQuery {
    type Output = usize;
}

pub struct GetCountElementsHandler {
    repository: Arc<dyn ElementRepositoryQueryPort>,
}

impl GetCountElementsHandler {
    pub fn new(repository: Arc<dyn ElementRepositoryQueryPort>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl QueryHandlerPort<GetCountElementsQuery> for GetCountElementsHandler {
    async fn handle(&self, _query: GetCountElementsQuery) -> Result<usize, Error> {
        self.repository.get_count().await
    }
}
