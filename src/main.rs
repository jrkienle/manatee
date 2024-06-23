#![cfg_attr(
    all(target_os = "windows", not(debug_assertions),),
    windows_subsystem = "windows"
)]

<<<<<<< Updated upstream
pub use manatee::game::Game;

pub fn main() {
    let game = Game::new();
    game.run();
=======
use manatee::{Game};

pub fn main() {
    // let mut game = Game::new();
    // let mut editor_scene = Scene::new();

    // editor_scene.spawn(components::Camera::new());
    // editor_scene.register_system(systems::CameraSystem::default());

    // game.load_scene(editor_scene);
    // game.run();

    Game::new("Manatee Editor").run(|game| {
        // game.new_scene();
    });
>>>>>>> Stashed changes
}
