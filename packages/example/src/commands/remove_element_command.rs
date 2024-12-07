use std::sync::Arc;

use async_trait::async_trait;
use kti_cqrs_rs::{errors::error::Error, ports::handler::command_handler_port::CommandHandlerPort};
use tokio::sync::RwLock;

use crate::contexts::app_context::AppContext;

pub struct RemoveElementCommand {
  pub element: i32,
}

#[async_trait]
impl CommandHandlerPort for RemoveElementCommand {
  type Context = Arc<RwLock<AppContext>>;
  type Output = ();

  async fn execute(&self, context: Self::Context) -> Result<Self::Output, Error> {
    let ctx = context.write().await;
    let command_repository = ctx.get_command_repository();

    command_repository.remove_element(self.element).await
  }
}
