use std::sync::Arc;

use async_trait::async_trait;

use crate::{errors::error::Error, ports::event_port::EventPort};

#[async_trait]
pub trait EventHandlerPort<Event: EventPort>: Send + Sync {
    async fn handle(&self, event: Event) -> Result<(), Error>;
}

pub type EventHandler<Event> = Arc<dyn EventHandlerPort<Event>>;
