const builtin = @import("builtin");
const std = @import("std");

const platform = switch (builtin.os.tag) {
    .macos => @import("./platforms/macos.zig"),
    else => @compileError(std.fmt.comptimePrint("Unsupported OS: {}", .{builtin.os.tag})),
};

pub usingnamespace platform;
