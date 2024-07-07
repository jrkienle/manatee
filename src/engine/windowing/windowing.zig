const builtin = @import("builtin");
const std = @import("std");

const platform = switch (builtin.os.tag) {
    .macos => @import("./platforms/macos.zig"),
    .windows => @import("./platforms/windows.zig"),
    else => @compileError(std.fmt.comptimePrint("Unsupported OS: {}", .{builtin.os.tag})),
};

pub fn init() void {
    platform.init();
}

pub fn create_window() void {
    platform.Window.init();
}

pub fn run() void {
    platform.run();
}
