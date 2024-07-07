const std = @import("std");

const WindowManager = @import("../windowing/windowing.zig").WindowManager;

pub const Game = struct {
    pub fn init() Game {
        return Game{};
    }

    pub fn run(self: *Game) *Game {
        var windowManager = WindowManager.init();
        _ = windowManager.create_window();
        _ = windowManager.run();
        return self;
    }
};
