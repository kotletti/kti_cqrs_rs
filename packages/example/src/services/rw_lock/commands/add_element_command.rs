use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use kti_cqrs_rs::common::handler::command_handler::CommandHandler;
use tokio::sync::RwLock;

use crate::services::rw_lock::contexts::rw_lock_context::RwLockContext;

pub struct Command {
  pub element: i32,
}

#[async_trait]
impl CommandHandler for Command {
  type Context = Arc<RwLock<RwLockContext>>;
  type Output = Result<(), Box<dyn Error>>;

  async fn execute(&self, context: Self::Context) -> Self::Output {
    let ctx = context.write().await;
    let command_repository = ctx.get_command_repository();

    command_repository.add_element(self.element).await
  }
}
