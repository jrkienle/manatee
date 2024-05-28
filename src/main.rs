#![cfg_attr(
    all(target_os = "windows", not(debug_assertions),),
    windows_subsystem = "windows"
)]

use manatee::{game::Game, scene::Scene};

pub fn main() {
    let mut game = Game::new();
    let editor_scene = Scene::new();
    game.load_scene(editor_scene);
    game.run();
}
