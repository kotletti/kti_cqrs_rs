use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use kti_cqrs_rs::common::handler::QueryHandler;
use tokio::sync::Mutex;

use crate::services::complex::contexts::complex_context::ComplexContext;

pub struct Query;

#[async_trait]
impl QueryHandler for Query {
  type Context = ComplexContext;
  type Output = Result<Vec<i32>, Box<dyn Error>>;

  async fn execute(&self, context: Arc<Mutex<Self::Context>>) -> Self::Output {
    let ctx = context.lock().await;
    let query_repository = ctx.get_query_repository();

    query_repository.get_elements().await
  }
}
