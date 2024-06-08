pub trait Component: Send + Sync + 'static {
    const TYPE_NAME: &'static str;

    fn type_name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
