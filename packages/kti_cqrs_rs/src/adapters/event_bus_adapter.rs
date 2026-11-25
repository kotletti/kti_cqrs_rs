use std::{
    any::{Any, TypeId, type_name},
    collections::HashMap,
    sync::Arc,
};

use crate::{
    errors::error::Error,
    ports::{
        bus::event_bus_port::EventBusPort,
        event_port::EventPort,
        handler::event_handler_port::{EventHandler, EventHandlerPort},
    },
};

#[derive(Default)]
pub struct EventBusAdapter {
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl EventBusAdapter {
    pub fn register<Event, Handler>(mut self, handler: Handler) -> Result<Self, Error>
    where
        Event: EventPort,
        Handler: EventHandlerPort<Event> + 'static,
    {
        let type_id = TypeId::of::<Event>();
        let type_name = type_name::<Event>();

        let handler: EventHandler<Event> = Arc::new(handler);

        if self.handlers.insert(type_id, Box::new(handler)).is_some() {
            return Err(Error::from(format!(
                "Command handler is already registered: {type_name}"
            )));
        }

        Ok(self)
    }
}

impl EventBusPort for EventBusAdapter {
    fn send<Event: EventPort + Sync>(&self, event: Event) {
        let type_id = TypeId::of::<Event>();
        let type_name = type_name::<Event>();

        let Some(handler) = self
            .handlers
            .get(&type_id)
            .and_then(|h| h.downcast_ref::<EventHandler<Event>>())
            .cloned()
        else {
            eprintln!("Event handler not found: {type_name}");

            return;
        };

        tokio::spawn(async move {
            let event = event.clone();

            handler.handle(event).await
        });
    }
}
