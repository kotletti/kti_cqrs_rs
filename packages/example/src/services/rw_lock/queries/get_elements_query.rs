use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use kti_cqrs_rs::common::handler::query_handler::QueryHandler;
use tokio::sync::RwLock;

use crate::services::rw_lock::contexts::rw_lock_context::RwLockContext;

pub struct Query;

#[async_trait]
impl QueryHandler for Query {
  type Context = Arc<RwLock<RwLockContext>>;
  type Output = Result<Vec<i32>, Box<dyn Error>>;

  async fn execute(&self, context: Self::Context) -> Self::Output {
    let ctx = context.read().await;
    let query_repository = ctx.get_query_repository();

    query_repository.get_elements().await
  }
}
