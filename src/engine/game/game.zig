const std = @import("std");

const gpu = @import("../gpu/gpu.zig");
const windowing = @import("../windowing/windowing.zig");

pub const Game = struct {
    arena: std.heap.ArenaAllocator,
    gpu_instance: gpu.GpuInstance,
    main_window: windowing.Window,
    swapchain: gpu.Swapchain,
    window_manager: windowing.WindowManager,
    pub fn init(allocator: ?std.mem.Allocator) !Game {
        // Zig recommends game loops use an arean allocator, however Zig also recommends that when
        // creating a library you allow developers to pass in their own allocators. Because I'm
        // generous and very cool, Manatee allows devs to optionally pass in their own, but creates
        // an arena allocator for those that don't want to use their own
        var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
        const allocator_or_default: std.mem.Allocator = allocator orelse arena.allocator();

        var window_manager = windowing.WindowManager.init();
        var main_window = windowing.Window.init(&window_manager);
        var gpu_instance = try gpu.GpuInstance.init(&main_window, allocator_or_default);
        const swapchain = try gpu.Swapchain.init(&gpu_instance, allocator_or_default);

        return Game{
            .arena = arena,
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
        self.swapchain.deinit();
        self.gpu_instance.deinit();
        self.main_window.deinit();
        self.window_manager.deinit();
        _ = self.arena.deinit();
        self.* = undefined;
    }
};
