const std = @import("std");
const win32 = @import("win32").everything;

pub const WindowManager = struct {
    pub fn init() WindowManager {
        std.debug.print("TODO: Implement WindowManager.init() for MacOS\n", .{});
        return WindowManager{};
    }

    pub fn run(self: *WindowManager) void {
        _ = self; // This will be used later
        std.debug.print("TODO: Implement WindowManager.run() for MacOS\n", .{});
    }

    pub fn deinit(self: *WindowManager) void {
        self.* = undefined;
    }
};

pub const Window = struct {
    pub fn init(window_manager: *WindowManager) Window {
        _ = window_manager; // This will be used later
        std.debug.print("TODO: Implement Window.init() for MacOS\n", .{});

        return Window{};
    }

    pub fn deinit(self: *Window) void {
        self.* = undefined;
    }
};
