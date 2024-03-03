use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use kti_cqrs_rs::common::handler::CommandHandler;
use tokio::sync::Mutex;

use crate::services::complex::contexts::complex_context::ComplexContext;

pub struct Command {
  pub element: i32,
}

#[async_trait]
impl CommandHandler for Command {
  type Context = ComplexContext;
  type Output = Result<(), Box<dyn Error>>;

  async fn execute(&self, context: Arc<Mutex<Self::Context>>) -> Self::Output {
    let ctx = context.lock().await;
    let command_repository = ctx.get_command_repository();

    command_repository.add_element(self.element).await
  }
}
