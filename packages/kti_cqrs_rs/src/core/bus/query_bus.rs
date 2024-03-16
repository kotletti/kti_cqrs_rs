use crate::common::handler::query_handler::QueryHandler;

#[derive(Clone, Copy)]
pub struct QueryBus;

impl QueryBus {
  pub async fn send<C, O>(
    &self,
    query: Box<dyn QueryHandler<Context = C, Output = O>>,
    context: C,
  ) -> O {
    query.execute(context).await
  }
}
