use crate::ports::{
  bus::event_bus_port::EventBusPort, handler::event_handler_port::EventHandlerPort,
};

#[derive(Clone, Copy)]
pub struct EventBusAdapter;

impl EventBusPort for EventBusAdapter {
  fn send<C: Send + 'static>(&self, event: Box<dyn EventHandlerPort<Context = C>>, context: C) {
    tokio::spawn(async move {
      std::mem::drop(event.execute(context).await);
    });
  }
}
