const builtin = @import("builtin");
const std = @import("std");

const platform = switch (builtin.os.tag) {
    .macos => @import("./platforms/macos.zig"),
    .windows => @import("./platforms/windows.zig"),
    else => @compileError(std.fmt.comptimePrint("Unsupported OS: {}", .{builtin.os.tag})),
};

pub const WindowManager = struct {
    pub fn init() WindowManager {
        platform.init();

        return WindowManager{};
    }

    pub fn create_window(self: *WindowManager) *WindowManager {
        platform.Window.init();
        return self;
    }

    pub fn run(self: *WindowManager) *WindowManager {
        platform.run();
        return self;
    }
};
