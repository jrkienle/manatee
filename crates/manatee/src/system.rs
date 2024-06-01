mod system_manager;
pub mod systems;

pub use system_manager::SystemManager;

pub trait System {
    fn on_create(&self) {}
}
