use std::{
    any::{Any, TypeId, type_name},
    collections::HashMap,
    sync::Arc,
};

use async_trait::async_trait;

use crate::{
    errors::error::Error,
    ports::{
        bus::query_bus_port::QueryBusPort,
        handler::query_handler_port::{QueryHandler, QueryHandlerPort},
        query_port::QueryPort,
    },
};

#[derive(Default)]
pub struct QueryBusAdapter {
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl QueryBusAdapter {
    pub fn register<Query, Handler>(mut self, handler: Handler) -> Result<Self, Error>
    where
        Query: QueryPort,
        Handler: QueryHandlerPort<Query> + 'static,
    {
        let type_id = TypeId::of::<Query>();
        let type_name = type_name::<Query>();

        let handler: QueryHandler<Query> = Arc::new(handler);

        if self.handlers.insert(type_id, Box::new(handler)).is_some() {
            return Err(Error::from(format!(
                "Query handler is already registered: {type_name}"
            )));
        }

        Ok(self)
    }
}

#[async_trait]
impl QueryBusPort for QueryBusAdapter {
    async fn send<Query: QueryPort>(&self, query: Query) -> Result<Query::Output, Error> {
        let type_id = TypeId::of::<Query>();
        let type_name = type_name::<Query>();

        let Some(handler) = self
            .handlers
            .get(&type_id)
            .and_then(|h| h.downcast_ref::<QueryHandler<Query>>())
        else {
            return Err(Error::from(format!("Query handler not found: {type_name}")));
        };

        handler.handle(query).await
    }
}
