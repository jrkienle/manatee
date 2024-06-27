use crate::{
    platform::{current_platform, Platform},
    window::{Window, WindowContext, WindowParams},
};
use hashbrown::HashMap;
use std::{borrow::BorrowMut, sync::Arc};

pub struct App {
    ctx: AppContext,
}

impl App {
    pub fn new() -> Self {
        let ctx = AppContext::new();
        Self { ctx }
    }

    pub fn run<F>(mut self, on_start: F)
    where
        F: 'static + FnOnce(&mut AppContext),
    {
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
    main_window_id: u16,
    next_window_id: u16,
    platform: Arc<dyn Platform>,
    windows: HashMap<u16, Window>,
}

impl AppContext {
    pub fn new() -> Self {
        let platform = current_platform();
        let windows = HashMap::new();
        Self {
            main_window_id: 0,
            next_window_id: 0,
            platform,
            windows,
        }
    }

    pub fn main_window(&mut self) -> &mut Window {
        self.windows
            .get_mut(&self.main_window_id)
            .expect("Main window not found")
    }

    pub fn new_window<F>(&mut self, params: WindowParams, on_open: F) 
    where
        F: 'static + FnOnce(&mut WindowContext),
    {
        let window_id = self.next_window_id;
        self.next_window_id += 1;
        let mut window = self.platform.new_window(params);
        on_open(&mut WindowContext::new(self, &mut window));

        self.windows
            .insert(window_id, window);

        self.windows.get_mut(&window_id).unwrap();
    }
}

impl Default for AppContext {
    fn default() -> Self {
        AppContext::new()
    }
}
