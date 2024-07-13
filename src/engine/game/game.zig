const std = @import("std");

const gpu = @import("../gpu/gpu.zig");
const windowing = @import("../windowing/windowing.zig");

pub const Game = struct {
    gpu_instance: gpu.Instance,
    window_manager: windowing.WindowManager,
    pub fn init() !Game {
        const window_manager = windowing.WindowManager.init();
        const gpu_instance = try gpu.Instance.init();
        return Game{
            .gpu_instance = gpu_instance,
            .window_manager = window_manager,
        };
    }

    pub fn run(self: *Game) !void {
        var window = windowing.Window.init(&self.window_manager);
        defer window.deinit();
        var surface = try gpu.Surface.init(&self.gpu_instance, self.window_manager, window);
        defer surface.deinit();
        self.window_manager.run();
    }

    pub fn deinit(self: *Game) void {
        self.window_manager.deinit();
        self.gpu_instance.deinit();
        self.* = undefined;
    }
};
