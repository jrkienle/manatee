const manatee = @import("./engine.zig");

pub fn main() !void {
    var game = manatee.Game{};
    _ = game.run();
}
