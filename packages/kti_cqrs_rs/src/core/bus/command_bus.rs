use std::sync::Arc;

use tokio::sync::Mutex;

use crate::common::handler::CommandHandler;

#[derive(Clone, Copy)]
pub struct CommandBus;

impl CommandBus {
  pub async fn send<C, O>(
    &self,
    command: Box<dyn CommandHandler<Context = C, Output = O>>,
    context: Arc<Mutex<C>>,
  ) -> O {
    command.execute(context).await
  }

  pub fn get_context() {}
}
