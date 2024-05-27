#![cfg_attr(
    all(target_os = "windows", not(debug_assertions),),
    windows_subsystem = "windows"
)]

pub use manatee::game::Game;

pub fn main() {
    let game = Game::new();
    game.run();
}
