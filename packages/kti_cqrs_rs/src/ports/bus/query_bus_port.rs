use async_trait::async_trait;

use crate::{errors::error::Error, ports::handler::query_handler_port::QueryHandlerPort};

#[derive(Clone, Copy)]
pub struct QueryBus;

#[async_trait]
pub trait QueryBusPort: Send + Sync {
  async fn send<C: Send, O>(
    &self,
    query: Box<dyn QueryHandlerPort<Context = C, Output = O>>,
    context: C,
  ) -> Result<O, Error>;
}
