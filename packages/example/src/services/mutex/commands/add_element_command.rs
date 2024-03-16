use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use kti_cqrs_rs::common::handler::command_handler::CommandHandler;
use tokio::sync::Mutex;

use crate::services::mutex::contexts::mutex_context::MutexContext;

pub struct Command {
  pub element: i32,
}

#[async_trait]
impl CommandHandler for Command {
  type Context = Arc<Mutex<MutexContext>>;
  type Output = Result<(), Box<dyn Error>>;

  async fn execute(&self, context: Self::Context) -> Self::Output {
    let ctx = context.lock().await;
    let command_repository = ctx.get_command_repository();

    command_repository.add_element(self.element).await
  }
}
