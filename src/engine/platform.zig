const builtin = @import("builtin");
const std = @import("std");

const platform = switch (builtin.os.tag) {
    .macos => @import("./platforms/macos.zig"),
    .windows => @import("./platforms/windows.zig"),
    else => @compileError(std.fmt.comptimePrint("Unsupported OS: {}", .{builtin.os.tag})),
};

pub usingnamespace platform;
