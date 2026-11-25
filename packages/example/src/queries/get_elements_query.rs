use std::sync::Arc;

use async_trait::async_trait;
use kti_cqrs_rs::{
    errors::error::Error,
    ports::{handler::query_handler_port::QueryHandlerPort, query_port::QueryPort},
};

use crate::ports::element_repository_query_port::ElementRepositoryQueryPort;

pub struct GetElementsQuery;

impl QueryPort for GetElementsQuery {
    type Output = Vec<i32>;
}

pub struct GetElementsHandler {
    repository: Arc<dyn ElementRepositoryQueryPort>,
}

impl GetElementsHandler {
    pub fn new(repository: Arc<dyn ElementRepositoryQueryPort>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl QueryHandlerPort<GetElementsQuery> for GetElementsHandler {
    async fn handle(&self, _query: GetElementsQuery) -> Result<Vec<i32>, Error> {
        self.repository.get_elements().await
    }
}
