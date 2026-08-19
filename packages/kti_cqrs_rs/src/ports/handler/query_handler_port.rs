use async_trait::async_trait;

#[async_trait]
pub trait QueryHandlerPort: Send + Sync {
    type Context;
    type Output;

    async fn execute(&self, context: Self::Context) -> Self::Output;
}

#[async_trait]
impl<H> QueryHandlerPort for Box<H>
where
    H: QueryHandlerPort + ?Sized,
    H::Context: Send + 'static,
    H::Output: Send + 'static,
{
    type Context = H::Context;
    type Output = H::Output;

    async fn execute(&self, context: Self::Context) -> Self::Output {
        (**self).execute(context).await
    }
}
