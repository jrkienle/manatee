const builtin = @import("builtin");
const std = @import("std");

const backend = switch (builtin.os.tag) {
    .windows => @import("./backends/vulkan.zig"),
    else => @compileError(std.fmt.comptimePrint("Unsupported OS: {}", .{builtin.os.tag})),
};

pub usingnamespace backend;
