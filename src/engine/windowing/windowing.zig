const builtin = @import("builtin");
const std = @import("std");

const platform = switch (builtin.os.tag) {
    // I'm not getting working autocomplete on windows with this plugged in, gotta love using an
    // alpha release language with no official tooling!
    // .macos => @import("./platforms/macos.zig"),
    .windows => @import("./platforms/windows.zig"),
    else => @compileError(std.fmt.comptimePrint("Unsupported OS: {}", .{builtin.os.tag})),
};

pub usingnamespace platform;
