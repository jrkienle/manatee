#![windows_subsystem = "windows"]
use std::str::FromStr;

pub use manatee::window::{WindowEventLoop, WindowEventLoopManager};

pub fn main() {
    let mut window_event_loop = WindowEventLoop::default();
    window_event_loop.start_event_loop();
    window_event_loop.create_window("Foo".to_string());
}
