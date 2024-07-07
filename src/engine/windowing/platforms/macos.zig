const std = @import("std");

pub fn init() void {
    // TODO: Implement MacOS Platform
    std.debug.print("TODO: Implement Platform.init() for MacOS\n", .{});
}

pub const Window = struct {
    pub fn init() void {
        std.debug.print("TODO: Implement Platform.Window.init() for MacOS\n", .{});
    }
};

pub fn run() void {
    std.debug.print("TODO: Implement Platform.run() for MacOS\n", .{});
}
