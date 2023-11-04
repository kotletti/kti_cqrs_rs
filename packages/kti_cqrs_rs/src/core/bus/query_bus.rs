use std::sync::{Arc, Mutex};

use crate::common::handler::QueryHandler;

#[derive(Clone, Copy)]
pub struct QueryBus;

impl QueryBus {
  pub async fn send<C, O>(
    &self,
    query: Box<dyn QueryHandler<Context = C, Output = O>>,
    context: Arc<Mutex<C>>,
  ) -> O {
    query.execute(context).await
  }
}
