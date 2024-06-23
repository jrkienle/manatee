mod ecs;
mod game;
mod graphics;
mod scene;

// Structs that will actually be used by people developing with Manatee
pub use ecs::{Component, System};
pub use game::{Context, Game, GameMode};
pub use scene::Scene;

// Pre-built components and systems to reduce end-user boilerplate
pub mod components;
pub mod systems;
