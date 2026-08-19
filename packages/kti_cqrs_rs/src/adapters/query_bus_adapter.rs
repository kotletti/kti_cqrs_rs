use async_trait::async_trait;

use crate::ports::{
    bus::query_bus_port::QueryBusPort, handler::query_handler_port::QueryHandlerPort,
};

#[derive(Clone, Copy)]
pub struct QueryBusAdapter;

#[async_trait]
impl QueryBusPort for QueryBusAdapter {
    async fn send<H>(&self, query: H, context: H::Context) -> H::Output
    where
        H: QueryHandlerPort + Send + 'static,
        H::Context: Send + 'static,
        H::Output: Send + 'static,
    {
        query.execute(context).await
    }
}
