use async_trait::async_trait;

use crate::ports::{
    bus::event_bus_port::EventBusPort, handler::event_handler_port::EventHandlerPort,
};

#[derive(Clone, Copy)]
pub struct EventBusAdapter;

#[async_trait]
impl EventBusPort for EventBusAdapter {
    async fn send<H>(&self, event: H, context: H::Context)
    where
        H: EventHandlerPort + Send + 'static,
        H::Context: Send + 'static,
    {
        if let Err(e) = event.execute(context).await {
            eprintln!("{e}");
        };
    }
}
