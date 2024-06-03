mod component_manager;
pub use component_manager::ComponentManager;

pub mod components;

// Bevvy had this and idk what it does, todo: learn go use google
pub trait Component: Send + Sync + 'static {
    fn type_name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

pub trait ComponentBundle: Send + Sync + 'static {}
