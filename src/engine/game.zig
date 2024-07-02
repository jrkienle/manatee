const std = @import("std");

pub const Game = struct {
    pub fn run(self: *Game) *Game {
        std.debug.print("Hello, Manatee!", .{});
        return self;
    }
};
