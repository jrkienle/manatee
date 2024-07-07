const std = @import("std");

const windowing = @import("./windowing/windowing.zig");

pub const Game = struct {
    pub fn init() Game {
        return Game{};
    }

    pub fn run(self: *Game) *Game {
        windowing.init();
        windowing.create_window();
        windowing.run();
        return self;
    }
};
