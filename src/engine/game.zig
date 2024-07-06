const std = @import("std");

const platform = @import("./platform.zig");

pub const Game = struct {
    pub fn init() Game {
        return Game{};
    }

    pub fn run(self: *Game) *Game {
        platform.init();
        platform.run();
        return self;
    }
};
