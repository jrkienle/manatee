use super::Platform;
use crate::window::{Window, WindowParams};
use raw_window_handle::Win32WindowHandle;
use std::num::NonZeroIsize;
use windows::{
    core::{w, HSTRING},
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Gdi::ValidateRect,
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, LoadCursorW,
            PostQuitMessage, RegisterClassW, ShowWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT,
            IDC_ARROW, MSG, SW_SHOW, WINDOW_EX_STYLE, WM_DESTROY, WM_PAINT, WNDCLASSW,
            WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
};

pub struct WindowsPlatform {}

impl WindowsPlatform {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for WindowsPlatform {
    fn default() -> Self {
        WindowsPlatform::new()
    }
}

impl Platform for WindowsPlatform {
    fn new_window(&self, params: WindowParams) -> Window {
        unsafe {
            let instance = GetModuleHandleW(None).unwrap();
            assert!(instance.0 != 0);

            let window_class = w!("Aquarium::Window");
            let wc = WNDCLASSW {
                hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
                hInstance: instance.into(),
                lpszClassName: window_class,

                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wndproc),
                ..Default::default()
            };

            let atom = RegisterClassW(&wc);
            assert!(atom != 0);

            // TODO: Eventually make more things than title configurable lol
            let raw_hwnd = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                window_class,
                &HSTRING::from(params.title),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                params.width.into(),
                params.height.into(),
                None,
                None,
                instance,
                None,
            );

            ShowWindow(raw_hwnd, SW_SHOW).unwrap();

            let window_handle = Win32WindowHandle::new(NonZeroIsize::new(raw_hwnd.0).unwrap());

            Window {}
        }
    }

    fn start(&self, on_start: Box<dyn 'static + FnOnce()>) {
        on_start();

        let mut message = MSG::default();
        unsafe {
            while GetMessageW(&mut message, None, 0, 0).into() {
                DispatchMessageW(&message);
            }
        }
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                ValidateRect(window, None).unwrap();
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
    }
}
