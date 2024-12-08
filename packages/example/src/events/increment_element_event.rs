use std::sync::Arc;

use async_trait::async_trait;
use kti_cqrs_rs::{errors::error::Error, ports::handler::event_handler_port::EventHandlerPort};
use tokio::sync::RwLock;

use crate::contexts::app_context::AppContext;

pub struct IncrementElementEvent {
  pub element: i32,
}

#[async_trait]
impl EventHandlerPort for IncrementElementEvent {
  type Context = Arc<RwLock<AppContext>>;

  async fn execute(&self, context: Self::Context) -> Result<(), Error> {
    let ctx = context.write().await;
    let command_repository = ctx.get_command_repository();

    command_repository
      .update_element(self.element, self.element + 1)
      .await?;

    Ok(())
  }
}
