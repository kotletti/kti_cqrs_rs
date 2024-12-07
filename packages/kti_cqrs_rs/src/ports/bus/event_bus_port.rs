use async_trait::async_trait;

use crate::ports::handler::event_handler_port::EventHandlerPort;

#[async_trait]
pub trait EventBusPort: Send + Sync {
  fn send<C: Send + 'static>(&self, event: Box<dyn EventHandlerPort<Context = C>>, context: C);
}
