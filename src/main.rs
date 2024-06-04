#![cfg_attr(
    all(target_os = "windows", not(debug_assertions),),
    windows_subsystem = "windows"
)]

use manatee::{components, systems, Game, Scene};

pub fn main() {
    let mut game = Game::new();
    let mut editor_scene = Scene::new();

    editor_scene.spawn(components::PositionComponent::new(0.0, 0.0, 0.0));
    editor_scene.register_system(systems::CameraSystem::new());

    game.load_scene(editor_scene);
    game.run();
}
