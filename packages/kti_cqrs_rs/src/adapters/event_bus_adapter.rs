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

pub struct EventBusAdapter {
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl EventBusAdapter {
    pub fn builder() -> EventBusBuilder {
        EventBusBuilder::default()
    }
}

#[derive(Default)]
pub struct EventBusBuilder {
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    duplicates: Vec<&'static str>,
}

impl EventBusBuilder {
    pub fn register<Event, Handler>(mut self, handler: Handler) -> Self
    where
        Event: EventPort,
        Handler: EventHandlerPort<Event> + 'static,
    {
        let type_id = TypeId::of::<Event>();
        let type_name = type_name::<Event>();

        let handler: EventHandler<Event> = Arc::new(handler);

        if self.handlers.insert(type_id, Box::new(handler)).is_some() {
            self.duplicates.push(type_name);
        }

        self
    }

    pub fn build(self) -> Result<EventBusAdapter, Error> {
        if !self.duplicates.is_empty() {
            return Err(Error::from(format!(
                "Event handlers are registered more than once: {}",
                self.duplicates.join(", ")
            )));
        }

        Ok(EventBusAdapter {
            handlers: self.handlers,
        })
    }
}

impl EventBusPort for EventBusAdapter {
    fn send<Event: EventPort>(&self, event: Event) {
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
            if let Err(e) = handler.handle(event).await {
                eprintln!("{type_name}: {e}");
            }
        });
    }
}
