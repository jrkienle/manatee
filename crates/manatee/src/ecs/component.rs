use downcast_rs::{Downcast,impl_downcast};

pub trait Component: Downcast + Send + Sync + 'static {
    fn type_name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
impl_downcast!(Component);
