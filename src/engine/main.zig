const std = @import("std");

pub const Game = @import("./game.zig").Game;

pub fn example() void {
    std.debug.print("Does this work?", .{});
}
