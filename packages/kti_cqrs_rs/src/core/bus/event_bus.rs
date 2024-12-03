use crate::common::handler::event_handler::EventHandler;

#[derive(Clone, Copy)]
pub struct EventBus;

impl EventBus {
  pub fn send<C: Send + 'static>(&self, event: Box<dyn EventHandler<Context = C>>, context: C) {
    tokio::spawn(async move {
      std::mem::drop(event.execute(context).await);
    });
  }
}
