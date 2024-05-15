use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy}, window::{Window, WindowId}};

#[derive(Debug, Clone, Copy)]
pub enum ManateeWindowEvent {
    CreateMainWindow
}

#[derive(Default)]
struct WinitApp {
    main_window: Option<Window>,
    // windows: HashMap<WindowId, WindowState>
}

impl ApplicationHandler::<ManateeWindowEvent> for WinitApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.main_window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: ManateeWindowEvent) {
        println!("User event: {event:?}");
        let _ = event_loop.create_window(Window::default_attributes());
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        // This will probably need to be a match after the app grows a bit
        if event == WindowEvent::CloseRequested {
            event_loop.exit();
        }
    }
}

#[derive(Default)]
pub struct WindowEventLoop {
    event_loop_proxy: Option<EventLoopProxy<ManateeWindowEvent>>,
}

pub trait WindowEventLoopManager {
    fn create_window(&mut self) {}
    fn send_event(&mut self, _event: ManateeWindowEvent) {}
    fn start_event_loop(&mut self) {}
}

impl WindowEventLoopManager for WindowEventLoop {
    fn create_window(&mut self) {
        self.send_event(ManateeWindowEvent::CreateMainWindow)
    }
    fn send_event(&mut self, event: ManateeWindowEvent) {
        let _ = self.event_loop_proxy.as_ref().unwrap().send_event(event);
    }
    fn start_event_loop(&mut self) {
        let event_loop = EventLoop::<ManateeWindowEvent>::with_user_event().build().unwrap();
        self.event_loop_proxy = Some(event_loop.create_proxy());

        let mut winit_app = WinitApp::default();
    
        let _ = event_loop.run_app(&mut winit_app);
    }
}