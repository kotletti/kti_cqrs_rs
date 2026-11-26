# Changelog

## Version 0.5.0 BREAKING CHANGES
* Split messages and handlers: a command, a query and an event are plain data, a handler is a separate type with its own dependencies
* Add `CommandPort`, `QueryPort` & `EventPort`
* Make handler ports generic over the message: `CommandHandlerPort<Command>`, `QueryHandlerPort<Query>` & `EventHandlerPort<Event>`
* Replace `execute(&self, context)` with `handle(&self, message)`
* Remove `Context` from handlers, buses and `ServiceBusPort`
* Move `Output` from handlers to `CommandPort` & `QueryPort`, it is the successful result now
* Return `Result<Output, Error>` from `handle` and `send` of commands & queries
* Make buses own their handlers
* Add `CommandBusBuilder`, `QueryBusBuilder` & `EventBusBuilder`: `builder()`, `register(...)`, `build()?`
* Report every message registered more than once as an error of `build`
* Report a command or a query without a handler as an error of `send`, log an event without a handler
* Make `EventBusPort::send` sync, `EventBusAdapter` spawns the handler on the tokio runtime and logs its error
* Remove `Clone` & `Copy` from bus adapters, they are created only by builders
* Remove blanket implementations of handler ports for `Box<H>`
* Add `CommandHandler`, `QueryHandler` & `EventHandler` aliases
* Add `tokio` to the library deps
* Rewrite the example on handlers with constructor injection
* Add tests for buses & `ServiceBusPort`

## Version 0.4.0 BREAKING CHANGES
* Remove `Result<O, Error>` from handler and bus signatures
* `CommandHandlerPort::execute` now returns `Self::Output`
* `QueryHandlerPort::execute` now returns `Self::Output`
* `EventHandlerPort::execute` still returns `Result<(), Error>` (error handling moved to adapter)
* The error type is now up to the user: put a `Result` into `Output`
* `errors::error::Error` is kept as a convenience alias
* Buses and `ServiceBusPort` take a handler by value instead of `Box<dyn ...>`
* `CommandBusPort::send` and `QueryBusPort::send` now return `H::Output`
* `ServiceBusPort::Context` now requires `Send + 'static`
* `ServiceBusPort::event` is now sync
* Add blanket `impl CommandHandlerPort for Box<H>`, same for `QueryHandlerPort` and `EventHandlerPort`
* `EventBusAdapter::send` now executes events synchronously via `await` instead of spawning detached tasks
* `EventBusAdapter` logs errors via `eprintln!` instead of requiring handlers to handle them internally
* Remove the leftover `QueryBus` struct (renamed to `QueryBusAdapter` back in 0.3.0)

## Version 0.3.0
* Implement async event bus
* `CommandHandler` -> `CommandHandlerPort`
* `QueryHandler` -> `QueryHandlerPort`
* `CommandBus` -> `CommandBusAdapter`
* `QueryBus` -> `QueryBusAdapter`
* Remove `Mutex` tests
* Update `RwLock` tests
* Add tests

## Version 0.2.0 BREAKING CHANGES
* Modify context for more flexible control of types
* Add example for `tokio::sync::Mutex`
* Add example for `tokio::sync::RwLock`

## Version 0.1.0 BREAKING CHANGES
* Replace the `std::sync::Mutex` to `tokio::sync::Mutex`
* Add new example for complex async tests

## Version 0.0.1
* Implement base logic commands & queries
* Add simple test cases
