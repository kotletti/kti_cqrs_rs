pub trait QueryPort: Send + 'static {
    type Output: Send;
}
