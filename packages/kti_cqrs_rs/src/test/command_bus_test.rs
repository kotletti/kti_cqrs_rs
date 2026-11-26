use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    adapters::command_bus_adapter::CommandBusAdapter,
    errors::error::Error,
    ports::{
        bus::command_bus_port::CommandBusPort, command_port::CommandPort,
        handler::command_handler_port::CommandHandlerPort,
    },
};

struct SumCommand(i32, i32);

impl CommandPort for SumCommand {
    type Output = i32;
}

struct GreetCommand(&'static str);

impl CommandPort for GreetCommand {
    type Output = String;
}

struct FailCommand;

impl CommandPort for FailCommand {
    type Output = ();
}

struct SumHandler;

#[async_trait]
impl CommandHandlerPort<SumCommand> for SumHandler {
    async fn handle(&self, command: SumCommand) -> Result<i32, Error> {
        Ok(command.0 + command.1)
    }
}

struct GreetHandler;

#[async_trait]
impl CommandHandlerPort<GreetCommand> for GreetHandler {
    async fn handle(&self, command: GreetCommand) -> Result<String, Error> {
        Ok(format!("Hello, {}", command.0))
    }
}

struct FailHandler;

#[async_trait]
impl CommandHandlerPort<FailCommand> for FailHandler {
    async fn handle(&self, _command: FailCommand) -> Result<(), Error> {
        Err(Error::from("Command failed"))
    }
}

#[tokio::test]
async fn should_route_command_to_its_handler() -> Result<(), Error> {
    let bus = CommandBusAdapter::builder()
        .register(SumHandler)
        .register(GreetHandler)
        .build()?;

    assert_eq!(bus.send(SumCommand(1, 2)).await?, 3);
    assert_eq!(bus.send(GreetCommand("Bob")).await?, "Hello, Bob");

    Ok(())
}

#[tokio::test]
async fn should_return_handler_error() -> Result<(), Error> {
    let bus = CommandBusAdapter::builder().register(FailHandler).build()?;

    let error = bus.send(FailCommand).await.unwrap_err();

    assert_eq!(error.to_string(), "Command failed");

    Ok(())
}

#[tokio::test]
async fn should_error_when_command_handler_is_not_registered() -> Result<(), Error> {
    let bus = CommandBusAdapter::builder()
        .register(GreetHandler)
        .build()?;

    let error = bus.send(SumCommand(1, 2)).await.unwrap_err().to_string();

    assert!(error.contains("Command handler not found"), "{error}");
    assert!(error.contains("SumCommand"), "{error}");

    Ok(())
}

#[test]
fn should_error_when_command_handler_is_registered_twice() -> Result<(), Error> {
    let error = CommandBusAdapter::builder()
        .register(SumHandler)
        .register(SumHandler)
        .build()
        .err()
        .ok_or("Expected an error")?
        .to_string();

    assert!(
        error.contains("Command handlers are registered more than once"),
        "{error}"
    );
    assert!(error.contains("SumCommand"), "{error}");

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn should_send_commands_concurrently() -> Result<(), Error> {
    let bus = Arc::new(CommandBusAdapter::builder().register(SumHandler).build()?);

    let tasks: Vec<_> = (0..100)
        .map(|i| {
            let bus = bus.clone();

            tokio::spawn(async move { bus.send(SumCommand(i, 1)).await })
        })
        .collect();

    for (i, task) in (0..100).zip(tasks) {
        assert_eq!(task.await??, i + 1);
    }

    Ok(())
}

#[test]
fn should_report_every_command_registered_twice() -> Result<(), Error> {
    let error = CommandBusAdapter::builder()
        .register(SumHandler)
        .register(GreetHandler)
        .register(SumHandler)
        .register(GreetHandler)
        .build()
        .err()
        .ok_or("Expected an error")?
        .to_string();

    assert!(error.contains("SumCommand"), "{error}");
    assert!(error.contains("GreetCommand"), "{error}");

    Ok(())
}
