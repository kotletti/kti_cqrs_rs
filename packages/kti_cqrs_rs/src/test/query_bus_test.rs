use async_trait::async_trait;

use crate::{
    adapters::query_bus_adapter::QueryBusAdapter,
    errors::error::Error,
    ports::{
        bus::query_bus_port::QueryBusPort, handler::query_handler_port::QueryHandlerPort,
        query_port::QueryPort,
    },
};

struct CountQuery;

impl QueryPort for CountQuery {
    type Output = usize;
}

struct NameQuery;

impl QueryPort for NameQuery {
    type Output = String;
}

struct FailQuery;

impl QueryPort for FailQuery {
    type Output = ();
}

struct CountHandler;

#[async_trait]
impl QueryHandlerPort<CountQuery> for CountHandler {
    async fn handle(&self, _query: CountQuery) -> Result<usize, Error> {
        Ok(7)
    }
}

struct NameHandler;

#[async_trait]
impl QueryHandlerPort<NameQuery> for NameHandler {
    async fn handle(&self, _query: NameQuery) -> Result<String, Error> {
        Ok("Bob".to_string())
    }
}

struct FailHandler;

#[async_trait]
impl QueryHandlerPort<FailQuery> for FailHandler {
    async fn handle(&self, _query: FailQuery) -> Result<(), Error> {
        Err(Error::from("Query failed"))
    }
}

#[tokio::test]
async fn should_route_query_to_its_handler() -> Result<(), Error> {
    let bus = QueryBusAdapter::builder()
        .register(CountHandler)
        .register(NameHandler)
        .build()?;

    assert_eq!(bus.send(CountQuery).await?, 7);
    assert_eq!(bus.send(NameQuery).await?, "Bob");

    Ok(())
}

#[tokio::test]
async fn should_return_handler_error() -> Result<(), Error> {
    let bus = QueryBusAdapter::builder().register(FailHandler).build()?;

    let error = bus.send(FailQuery).await.unwrap_err();

    assert_eq!(error.to_string(), "Query failed");

    Ok(())
}

#[tokio::test]
async fn should_error_when_query_handler_is_not_registered() -> Result<(), Error> {
    let bus = QueryBusAdapter::builder().register(NameHandler).build()?;

    let error = bus.send(CountQuery).await.unwrap_err().to_string();

    assert!(error.contains("Query handler not found"), "{error}");
    assert!(error.contains("CountQuery"), "{error}");

    Ok(())
}

#[test]
fn should_error_when_query_handler_is_registered_twice() -> Result<(), Error> {
    let error = QueryBusAdapter::builder()
        .register(CountHandler)
        .register(CountHandler)
        .build()
        .err()
        .ok_or("Expected an error")?
        .to_string();

    assert!(
        error.contains("Query handlers are registered more than once"),
        "{error}"
    );
    assert!(error.contains("CountQuery"), "{error}");

    Ok(())
}

#[test]
fn should_report_every_query_registered_twice() -> Result<(), Error> {
    let error = QueryBusAdapter::builder()
        .register(CountHandler)
        .register(NameHandler)
        .register(CountHandler)
        .register(NameHandler)
        .build()
        .err()
        .ok_or("Expected an error")?
        .to_string();

    assert!(error.contains("CountQuery"), "{error}");
    assert!(error.contains("NameQuery"), "{error}");

    Ok(())
}
