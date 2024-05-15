#![windows_subsystem = "windows"]
pub use manatee::window::create_and_run_window;

pub fn main() {
    create_and_run_window();
}
