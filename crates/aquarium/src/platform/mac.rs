use super::Platform;
use crate::window::{Window, WindowParams};
use cocoa::{
    appkit::{
        NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
        NSApplicationActivationPolicyRegular, NSBackingStoreBuffered, NSRunningApplication,
        NSWindow, NSWindowStyleMask,
    },
    base::{nil, NO},
    foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize, NSString},
};
use objc::runtime::Object;

pub struct MacPlatform {
    app: *mut Object,
}

impl MacPlatform {
    pub fn new() -> Self {
        let _pool = unsafe { NSAutoreleasePool::new(nil) };
        let app = unsafe { NSApp() };
        unsafe { app.setActivationPolicy_(NSApplicationActivationPolicyRegular) };
        Self { app }
    }
}

impl Default for MacPlatform {
    fn default() -> Self {
        MacPlatform::new()
    }
}

impl Platform for MacPlatform {
    fn new_window(&self, params: WindowParams) -> Window {
        let window = unsafe {
            NSWindow::alloc(nil)
                .initWithContentRect_styleMask_backing_defer_(
                    NSRect::new(
                        NSPoint::new(0., 0.),
                        NSSize::new(params.width as f64, params.height as f64),
                    ),
                    NSWindowStyleMask::NSTitledWindowMask
                        | NSWindowStyleMask::NSClosableWindowMask
                        | NSWindowStyleMask::NSResizableWindowMask
                        | NSWindowStyleMask::NSMiniaturizableWindowMask,
                    NSBackingStoreBuffered,
                    NO,
                )
                .autorelease()
        };
        unsafe { window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.)) };
        unsafe { window.center() };
        let title = unsafe { NSString::alloc(nil).init_str(params.title) };
        unsafe { window.setTitle_(title) };
        unsafe { window.makeKeyAndOrderFront_(nil) };
        Window {}
    }

    fn start(&self, on_start: Box<dyn 'static + FnOnce()>) {
        on_start();

        let current_app = unsafe { NSRunningApplication::currentApplication(nil) };
        unsafe { current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps) };
        unsafe { self.app.run() };
    }
}
