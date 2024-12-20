# Changelog

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
