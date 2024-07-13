const std = @import("std");

const manatee = @import("manatee");

pub fn main() !void {
    var game = try manatee.game.Game.init();
    defer game.deinit();
    _ = try game.run();
}
