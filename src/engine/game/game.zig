const std = @import("std");

const gpu = @import("../gpu/gpu.zig");
const windowing = @import("../windowing/windowing.zig");

// const renderVulkanTriangleExample = @import("./vulkan_triangle_example.zig").renderVulkanTriangleExample;

pub const Game = struct {
    arena: std.heap.ArenaAllocator,
    allocator: std.mem.Allocator,
    gpu_instance: gpu.GpuInstance,
    main_window: windowing.Window,
    window_manager: windowing.WindowManager,
    pub fn init(allocator: ?std.mem.Allocator) !Game {
        // Zig recommends game loops use an arean allocator, however Zig also recommends that when
        // creating a library you allow developers to pass in their own allocators. Because I'm
        // generous and very cool, Manatee allows devs to optionally pass in their own, but creates
        // an arena allocator for those that don't want to use their own. Note that this allocator
        // will be created and passed into the game struct regardless of whether or not a custom
        // allocator is passed in, this is done so that Manatee doesn't have to guess wherher or
        // not to deinit the allocator during the Game's deinit function
        var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
        const allocator_or_default: std.mem.Allocator = allocator orelse arena.allocator();

        var window_manager = windowing.WindowManager.init();
        // TODO: Should we allow multiple windows to be created? If so, how would I deal with
        // attaching multiple GPU instances? I can see a good use case here for applications such
        // as the editor itself, but not really for actual games? Idk
        var main_window = windowing.Window.init(&window_manager);
        const gpu_instance = try gpu.GpuInstance.init(&main_window, allocator_or_default);

        return Game{
            .allocator = allocator_or_default,
            .arena = arena,
            .gpu_instance = gpu_instance,
            .main_window = main_window,
            .window_manager = window_manager,
        };
    }

    pub fn run(self: *Game) !void {
        // TODO: Obviously don't render the example in the final build and let devs render their
        // actual game lmao. Also the Swapchain code should be abstracted and made into a part of
        // the GPU instance to help future proof things for different GPU backends
        _ = try gpu.Swapchain.init(&self.gpu_instance, self.allocator);
        //  _ = try renderVulkanTriangleExample(&self.gpu_instance, &self.swapchain, &self.allocator);
        self.window_manager.run();
    }

    pub fn deinit(self: *Game) void {
        self.gpu_instance.deinit();
        self.main_window.deinit();
        self.window_manager.deinit();
        _ = self.arena.deinit();
        self.* = undefined;
    }
};
