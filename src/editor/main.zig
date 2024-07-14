const std = @import("std");

const manatee = @import("manatee");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var game = try manatee.game.Game.init(allocator);
    defer game.deinit();
    _ = try game.run();
}
