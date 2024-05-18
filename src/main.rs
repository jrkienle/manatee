#![windows_subsystem = "windows"]

pub use manatee::game::Game;

pub fn main() {
    Game::default().start(|| {
        print!("Does this even work?");
    });
}
