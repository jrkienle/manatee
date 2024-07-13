const std = @import("std");

const gpu = @import("../gpu/gpu.zig");
const windowing = @import("../windowing/windowing.zig");

pub const Game = struct {
    gpu_instance: gpu.GpuInstance,
    main_window: windowing.Window,
    swapchain: gpu.Swapchain,
    window_manager: windowing.WindowManager,
    pub fn init() !Game {
        var window_manager = windowing.WindowManager.init();
        var main_window = windowing.Window.init(&window_manager);
        var gpu_instance = try gpu.GpuInstance.init(&main_window);
        const swapchain = try gpu.Swapchain.init(&gpu_instance);

        return Game{
            .gpu_instance = gpu_instance,
            .main_window = main_window,
            .swapchain = swapchain,
            .window_manager = window_manager,
        };
    }

    pub fn run(self: *Game) !void {
        self.window_manager.run();
    }

    pub fn deinit(self: *Game) void {
        self.gpu_instance.deinit();
        self.main_window.deinit();
        self.window_manager.deinit();
        self.* = undefined;
    }
};
