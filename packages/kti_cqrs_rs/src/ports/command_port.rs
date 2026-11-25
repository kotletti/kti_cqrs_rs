pub trait CommandPort: Send + 'static {
    type Output: Send;
}
