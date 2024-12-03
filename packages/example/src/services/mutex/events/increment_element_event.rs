use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use kti_cqrs_rs::common::handler::event_handler::EventHandler;
use tokio::sync::Mutex;

use crate::services::mutex::contexts::mutex_context::MutexContext;

pub struct Event {
  pub element: i32,
}

#[async_trait]
impl EventHandler for Event {
  type Context = Arc<Mutex<MutexContext>>;

  async fn execute(&self, context: Self::Context) -> Result<(), Box<dyn Error>> {
    let ctx = context.lock().await;
    let command_repository = ctx.get_command_repository();

    command_repository
      .update_element(self.element, self.element + 1)
      .await?;

    Ok(())
  }
}
