use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use kti_cqrs_rs::common::handler::query_handler::QueryHandler;
use tokio::sync::Mutex;

use crate::services::mutex::contexts::mutex_context::MutexContext;

pub struct Query;

#[async_trait]
impl QueryHandler for Query {
  type Context = Arc<Mutex<MutexContext>>;
  type Output = Result<i32, Box<dyn Error>>;

  async fn execute(&self, context: Self::Context) -> Self::Output {
    let ctx = context.lock().await;
    let query_repository = ctx.get_query_repository();

    query_repository.get_count().await
  }
}
