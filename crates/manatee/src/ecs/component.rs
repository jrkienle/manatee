pub trait Component: Send + Sync + 'static {
    fn type_name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
