#[cfg(target_os = "macos")]
mod mac;
#[cfg(target_os = "windows")]
mod windows;

use crate::window::{Window, WindowParams};
use std::sync::Arc;

#[cfg(target_os = "macos")]
use mac::MacPlatform;

#[cfg(target_os = "windows")]
use windows::WindowsPlatform;

pub trait Platform {
    fn new_window(&self, params: WindowParams) -> Window;

    fn start(&self, on_start: Box<dyn 'static + FnOnce()>);
}

#[cfg(target_os = "windows")]
pub fn current_platform() -> Arc<dyn Platform> {
    Arc::new(WindowsPlatform::new())
}

#[cfg(target_os = "macos")]
pub fn current_platform() -> Arc<dyn Platform> {
    Arc::new(MacPlatform::new())
}
