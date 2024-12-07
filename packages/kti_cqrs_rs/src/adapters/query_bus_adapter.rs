use async_trait::async_trait;

use crate::{
  errors::error::Error,
  ports::{bus::query_bus_port::QueryBusPort, handler::query_handler_port::QueryHandlerPort},
};

#[derive(Clone, Copy)]
pub struct QueryBusAdapter;

#[async_trait]
impl QueryBusPort for QueryBusAdapter {
  async fn send<C: Send, O>(
    &self,
    query: Box<dyn QueryHandlerPort<Context = C, Output = O>>,
    context: C,
  ) -> Result<O, Error> {
    query.execute(context).await
  }
}
