use std::sync::Arc;

use async_trait::async_trait;
use kti_cqrs_rs::{errors::error::Error, ports::handler::command_handler_port::CommandHandlerPort};
use tokio::sync::RwLock;

use crate::contexts::app_context::AppContext;

pub struct AddElementCommand {
    pub element: i32,
}

#[async_trait]
impl CommandHandlerPort for AddElementCommand {
    type Context = Arc<RwLock<AppContext>>;
    type Output = Result<(), Error>;

    async fn execute(&self, context: Self::Context) -> Self::Output {
        let ctx = context.read().await;
        let command_repository = ctx.get_command_repository();

        command_repository.add_element(self.element).await
    }
}
