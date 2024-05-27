use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

#[derive(Debug, Clone)]
pub enum ManateeWindowEvent {
    CreateWindow { title: String },
    SetMainWindowParams { title: Option<String> },
}

pub fn start_window_event_loop<F>(on_start: F)
where
    F: FnOnce(),
{
    loop {
        on_start();
    }
}
