use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy}, raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle}, window::{Window, WindowId}};

#[derive(Debug, Clone)]
pub enum ManateeWindowEvent {
    CreateWindow {
        title: String
    },
    SetMainWindowParams {
        title: Option<String>
    }
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
        match event {
            ManateeWindowEvent::CreateWindow { title } => {
                let parent_window = self.main_window.as_ref().unwrap().raw_window_handle().unwrap();
                let mut child_window_attributes = Window::default_attributes().with_title(title);
                child_window_attributes = unsafe { child_window_attributes.with_parent_window(Some(parent_window)) };
                // child_window_attributes = unsafe child_window_attributes.with_parent_window((Some(self.main_window.unwrap().raw_window_handle())));
                let _ = event_loop.create_window(child_window_attributes);
            },
            ManateeWindowEvent::SetMainWindowParams { title } => {
                if (Option::is_some(&title)) {
                    self.main_window.as_ref().unwrap().set_title(title.unwrap().as_str())
                }
            },
            _ => (),
        }
        // let _ = event_loop.create_window(Window::default_attributes());
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
    fn create_window(&mut self, _title: String) {}
    fn send_event(&mut self, _event: ManateeWindowEvent) {}
    fn start_event_loop(&mut self) {}
}

impl WindowEventLoopManager for WindowEventLoop {
    fn create_window(&mut self, title: String) {
        self.send_event(ManateeWindowEvent::CreateWindow { title })
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