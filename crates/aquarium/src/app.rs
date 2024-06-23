use crate::{platform::{current_platform, Platform}, window::{Window,WindowParams}};
use std::{borrow::BorrowMut,sync::Arc};

pub struct App {
    ctx: AppContext,
}

impl App {
    pub fn new() -> Self {
        let ctx = AppContext::new();
        Self {
            ctx
        }
    }

    pub fn run<F>(mut self, on_start: F)
    where F: 'static + FnOnce(&mut AppContext) {
        let platform = self.ctx.platform.clone();
        platform.start(Box::new(move || {
            on_start(&mut *self.ctx.borrow_mut());
        }));
    }
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

pub struct AppContext {
    pub active_window: Option<Window>,
    pub(crate) platform: Arc<dyn Platform>
}

impl AppContext {
    pub fn new() -> Self {
        let platform = current_platform();
        Self {
            active_window: None,
            platform
        }
    }

    pub fn new_window(&mut self, params: WindowParams) {
        self.active_window = Some(self.platform.new_window(params));
    }
}

impl Default for AppContext {
    fn default() -> Self {
        AppContext::new()
    }
}
