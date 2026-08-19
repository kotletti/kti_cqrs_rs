use async_trait::async_trait;

use crate::errors::error::Error;

#[async_trait]
pub trait EventHandlerPort: Send + Sync {
    type Context;

    async fn execute(&self, context: Self::Context) -> Result<(), Error>;
}

#[async_trait]
impl<H> EventHandlerPort for Box<H>
where
    H: EventHandlerPort + ?Sized,
    H::Context: Send + 'static,
{
    type Context = H::Context;

    async fn execute(&self, context: Self::Context) -> Result<(), Error> {
        (**self).execute(context).await
    }
}
