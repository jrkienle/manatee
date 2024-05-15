use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy},
    window::{Window, WindowId},
};

#[derive(Default)]
pub struct WinitApp {
    main_window: Option<Window>,
}

impl ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.main_window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        // This will probably need to be a match after the app grows a bit
        if event == WindowEvent::CloseRequested {
            event_loop.exit();
        }
    }
}

pub fn create_winit_event_loop() -> EventLoop<()> {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for a game engine
    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop
}