const std = @import("std");

const manatee = @import("manatee");

pub fn main() !void {
    var game = manatee.Game.init();
    defer game.deinit();
    _ = game.run();
}
