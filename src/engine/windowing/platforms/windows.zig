const std = @import("std");
const win32 = @import("win32").everything;

var hInstance: win32.HINSTANCE = undefined;

pub fn init() void {
    hInstance = win32.GetModuleHandleW(null).?;
}

pub const Window = struct {
    // TODO: Return some generic window obj to manipulate window
    pub fn init() void {
        const CLASS_NAME = win32.L("Manatee");
        const wc = win32.WNDCLASSEXW{
            .cbSize = @sizeOf(win32.WNDCLASSEXW),
            .style = win32.WNDCLASS_STYLES{},
            .lpfnWndProc = WindowProc,
            .cbClsExtra = 0,
            .cbWndExtra = 0,
            .hInstance = hInstance,
            .hIcon = null,
            .hCursor = null,
            .hbrBackground = null,
            .lpszMenuName = null,
            .lpszClassName = CLASS_NAME,
            .hIconSm = null,
        };

        _ = win32.RegisterClassExW(&wc);

        // I hate that the Zig standard formatter won't let me add line breaks to func calls lol
        const hwnd = win32.CreateWindowExW(.{}, CLASS_NAME, win32.L("Manatee Game Engine Window"), win32.WS_VISIBLE, win32.CW_USEDEFAULT, win32.CW_USEDEFAULT, 400, 200, null, null, hInstance, null);

        _ = win32.ShowWindow(hwnd, win32.SW_SHOW);
    }
};

pub fn run() void {
    var msg: win32.MSG = undefined;
    while (win32.GetMessageW(&msg, null, 0, 0) != 0) {
        _ = win32.TranslateMessage(&msg);
        _ = win32.DispatchMessageW(&msg);
    }
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
            var ps: win32.PAINTSTRUCT = undefined;
            const hdc = win32.BeginPaint(hwnd, &ps);
            _ = win32.FillRect(hdc, &ps.rcPaint, @ptrFromInt(@intFromEnum(win32.COLOR_WINDOW) + 1));
            _ = win32.TextOutW(hdc, 20, 20, win32.L("TODO: Attach Vulkan and D3D12 Renderers to Window"), 49);
            _ = win32.EndPaint(hwnd, &ps);
            return 0;
        },
        else => {
            return win32.DefWindowProcW(hwnd, uMsg, wParam, lParam);
        },
    }
}
