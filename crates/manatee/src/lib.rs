mod component;
mod entity;
mod game;
mod scene;
mod system;

// Structs that will actually be used by people developing with Manatee
pub use component::Component;
pub use entity::Entity;
pub use game::{Game, GameMode};
pub use scene::{Scene, SceneManager};
pub use system::System;

// Pre-built components and systems to reduce end-user boilerplate
pub use component::components;
pub use system::systems;
