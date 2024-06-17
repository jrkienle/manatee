#![cfg_attr(
    all(target_os = "windows", not(debug_assertions),),
    windows_subsystem = "windows"
)]

use manatee::{components, systems, Game, Scene};

pub fn main() {
    let mut game = Game::new();
    let mut editor_scene = Scene::new();

    // TODO: Add support for bundles so developers can add multiple components to an entity with a
    // single call. This will also be essential for building the editor app (read: the current app)
    let player = editor_scene.spawn(components::Camera::default());
    editor_scene.add_component(&player, components::Transform::default());

    editor_scene.spawn(components::WgpuTutorial::default());

    editor_scene.register_system(systems::CameraSystem::default());
    editor_scene.register_system(systems::WgpuTutorialSystem::default());

    game.load_scene(editor_scene);
    game.run();
}
