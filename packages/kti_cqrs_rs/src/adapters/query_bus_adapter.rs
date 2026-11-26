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
pub struct QueryBusAdapter {
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl QueryBusAdapter {
    pub fn builder() -> QueryBusBuilder {
        QueryBusBuilder::default()
    }
}

#[derive(Default)]
pub struct QueryBusBuilder {
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    duplicates: Vec<&'static str>,
}

impl QueryBusBuilder {
    pub fn register<Query, Handler>(mut self, handler: Handler) -> Self
    where
        Query: QueryPort,
        Handler: QueryHandlerPort<Query> + 'static,
    {
        let type_id = TypeId::of::<Query>();
        let type_name = type_name::<Query>();

        let handler: QueryHandler<Query> = Arc::new(handler);

        if self.handlers.insert(type_id, Box::new(handler)).is_some() {
            self.duplicates.push(type_name);
        }

        self
    }

    pub fn build(self) -> Result<QueryBusAdapter, Error> {
        if !self.duplicates.is_empty() {
            return Err(Error::from(format!(
                "Query handlers are registered more than once: {}",
                self.duplicates.join(", ")
            )));
        }

        Ok(QueryBusAdapter {
            handlers: self.handlers,
        })
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
