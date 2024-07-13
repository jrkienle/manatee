const std = @import("std");
const win32 = @import("win32").everything;

pub const WindowManager = struct {
    hInstance: win32.HINSTANCE,

    pub fn init() WindowManager {
        const hInstance = win32.GetModuleHandleW(null).?;
        return WindowManager{ .hInstance = hInstance };
    }

    pub fn run(self: *WindowManager) void {
        _ = self;
        var msg: win32.MSG = undefined;
        while (win32.GetMessageW(&msg, null, 0, 0) != 0) {
            _ = win32.TranslateMessage(&msg);
            _ = win32.DispatchMessageW(&msg);
        }
    }

    pub fn deinit(self: *WindowManager) void {
        self.* = undefined;
    }
};

pub const Window = struct {
    hInstance: win32.HINSTANCE,
    hwnd: win32.HWND,

    pub fn init(window_manager: *WindowManager) Window {
        const CLASS_NAME = win32.L("Manatee");
        const wc = win32.WNDCLASSEXW{
            .cbSize = @sizeOf(win32.WNDCLASSEXW),
            .style = win32.WNDCLASS_STYLES{},
            .lpfnWndProc = WindowProc,
            .cbClsExtra = 0,
            .cbWndExtra = 0,
            .hInstance = @ptrCast(window_manager.hInstance),
            .hIcon = null,
            .hCursor = null,
            .hbrBackground = null,
            .lpszMenuName = null,
            .lpszClassName = CLASS_NAME,
            .hIconSm = null,
        };

        _ = win32.RegisterClassExW(&wc);

        // I hate that the Zig standard formatter won't let me add line breaks to func calls lol
        const hwnd = win32.CreateWindowExW(win32.WS_EX_OVERLAPPEDWINDOW, CLASS_NAME, win32.L("Manatee Game Engine Window"), win32.WS_OVERLAPPEDWINDOW, win32.CW_USEDEFAULT, win32.CW_USEDEFAULT, 400, 200, null, null, window_manager.hInstance, null);

        _ = win32.ShowWindow(hwnd, win32.SW_SHOW);

        return Window{
            .hInstance = window_manager.hInstance,
            .hwnd = hwnd.?,
        };
    }

    pub fn deinit(self: *Window) void {
        self.* = undefined;
    }

    fn WindowProc(
        hwnd: win32.HWND,
        uMsg: u32,
        wParam: win32.WPARAM,
        lParam: win32.LPARAM,
    ) callconv(std.os.windows.WINAPI) win32.LRESULT {
        switch (uMsg) {
            win32.WM_DESTROY => {
                win32.PostQuitMessage(0);
                return 0;
            },
            win32.WM_PAINT => {
                return win32.ValidateRect(hwnd, null);
            },
            else => {
                return win32.DefWindowProcW(hwnd, uMsg, wParam, lParam);
            },
        }
    }
};
