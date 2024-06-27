use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle};

use crate::AppContext;

#[derive(Clone, Copy)]
pub struct Window {
    pub(crate) display_handle: DisplayHandle<'static>,
    pub height: u32,
    pub(crate) window_handle: WindowHandle<'static>,
    pub width: u32,
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}

impl HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(self.display_handle)
    }
}

impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        Ok(self.window_handle)
    }
}

pub struct WindowParams {
    pub height: u32,
    pub title: &'static str,
    pub width: u32,
}

impl Default for WindowParams {
    fn default() -> Self {
        Self {
            height: 600,
            title: "Aquarium Window",
            width: 800,
        }
    }
}

pub struct WindowContext<'a> {
    pub app: &'a mut AppContext,
    pub window: &'a mut Window
}

impl<'a> WindowContext<'a> {
    pub fn new(app: &'a mut AppContext, window: &'a mut Window) -> Self {
        Self {
            app,
            window
        }
    }
}