#![cfg_attr(
    all(target_os = "windows", not(debug_assertions),),
    windows_subsystem = "windows"
)]

use manatee::{components::CameraComponent, systems::CameraSystem, Entity, Game, Scene};

pub fn main() {
    let mut game = Game::new();

    let mut camera = CameraComponent::new();
    let player = Entity::new("Player");
    player.add_component(&mut camera);

    let mut editor_scene = Scene::new();

    let camera_system = CameraSystem {};

    editor_scene.register_system(camera_system);
    game.load_scene(editor_scene);
    game.run();
}
