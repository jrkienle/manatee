// #![windows_subsystem = "windows"]

pub use manatee::{game::Game, scene::Scene};

pub fn main() {
    let mut game = Game::new();
    let test_scene = Scene {};
    game.load_scene(test_scene);
    game.run();
}
