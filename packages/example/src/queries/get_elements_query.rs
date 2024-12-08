use std::sync::Arc;

use async_trait::async_trait;
use kti_cqrs_rs::{errors::error::Error, ports::handler::query_handler_port::QueryHandlerPort};
use tokio::sync::RwLock;

use crate::contexts::app_context::AppContext;

pub struct GetElementsQuery;

#[async_trait]
impl QueryHandlerPort for GetElementsQuery {
  type Context = Arc<RwLock<AppContext>>;
  type Output = Vec<i32>;

  async fn execute(&self, context: Self::Context) -> Result<Self::Output, Error> {
    let ctx = context.read().await;
    let query_repository = ctx.get_query_repository();

    query_repository.get_elements().await
  }
}
