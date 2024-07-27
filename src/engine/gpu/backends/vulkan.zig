const builtin = @import("builtin");
const std = @import("std");
const vk = @import("vulkan");

const windowing = @import("../../windowing/windowing.zig");

// TODO: Is there a way to fit this into the instance struct somehow?
var vulkan_lib: std.DynLib = undefined;

// Vulkan requires you to select each feature you want manually and add it to this array
// TODO: Figure out what features I actually need
const apis: []const vk.ApiInfo = &.{
    vk.features.version_1_0,
    vk.features.version_1_1,
    vk.features.version_1_2,
    vk.features.version_1_3,
    vk.extensions.khr_surface,
    vk.extensions.khr_swapchain,
    vk.extensions.khr_win_32_surface,
};

// These structs are actually just typings which blew my mind when first trying to understand the
// Vulkan Zig library and Zig itself. They effectively map the above APIs to the base Vulkan
// typings so we can get smarter autocomplete and compilation with Vulkan
const BaseDispatch = vk.BaseWrapper(apis);
const Instance = vk.InstanceProxy(apis);
const InstanceDispatch = vk.InstanceWrapper(apis);
const Device = vk.DeviceProxy(apis);
const DeviceDispatch = vk.DeviceWrapper(apis);
const DeviceCandidate = struct {
    physical_device: vk.PhysicalDevice,
    physical_properties: vk.PhysicalDeviceProperties,
    queue_index_graphics: u32,
    queue_index_present: u32,
};

pub const GpuInstance = struct {
    allocator: std.mem.Allocator,
    device: Device,
    instance: Instance,
    physical_device: vk.PhysicalDevice,
    queue_graphics: Queue,
    queue_present: Queue,
    surface: vk.SurfaceKHR,

    pub fn init(window: *windowing.Window, allocator: std.mem.Allocator) !GpuInstance {
        vulkan_lib = try std.DynLib.open(switch (builtin.target.os.tag) {
            .windows => "vulkan-1.dll",
            // TODO: I want to add Vulkan support to Mac and Linux in the future, but the MVP of
            // this engine will likely be Windows-only
            // .linux => "libvulkan.so.1",
            // .macos => "libvulkan.1.dylib",
            else => @compileError(std.fmt.comptimePrint("Unsupported OS: {}", .{builtin.os.tag})),
        });
        const base_dispatch = try BaseDispatch.load(getInstanceProcAddress);

        const instance_handle = try createInstanceHandle(base_dispatch);
        const instance_dispatch = try allocator.create(InstanceDispatch);
        errdefer allocator.destroy(instance_dispatch);
        instance_dispatch.* = try InstanceDispatch.load(instance_handle, base_dispatch.dispatch.vkGetInstanceProcAddr);
        const instance = createInstance(instance_handle, instance_dispatch);
        errdefer instance.destroyInstance(null);

        const surface = try createSurface(instance, window);
        errdefer instance.destroySurfaceKHR(surface, null);

        const best_physical_device = try pickPhysicalDevices(instance, surface, allocator);
        const device_handle = try createDeviceHandle(instance, best_physical_device);
        const device_dispatch = try allocator.create(DeviceDispatch);
        errdefer allocator.destroy(device_dispatch);
        device_dispatch.* = try DeviceDispatch.load(device_handle, instance_dispatch.dispatch.vkGetDeviceProcAddr);
        const device = createDevice(device_handle, device_dispatch);
        errdefer device.destroyDevice(null);

        const queue_graphics = Queue.init(device, best_physical_device.queue_index_graphics);
        const queue_present = Queue.init(device, best_physical_device.queue_index_present);

        return GpuInstance{
            .allocator = allocator,
            // .base_dispatch = base_dispatch,
            .instance = instance,
            // .instance_dispatch = instance_dispatch,
            .device = device,
            .physical_device = best_physical_device.physical_device,
            // .device_dispatch = device_dispatch,
            .queue_graphics = queue_graphics,
            .queue_present = queue_present,
            .surface = surface,
        };
    }

    pub fn deinit(self: *GpuInstance) void {
        // This may seem obvious, but things are intentially destroyed in the opposite order of
        // their creation
        self.device.destroyDevice(null);
        self.instance.destroySurfaceKHR(self.surface, null);
        self.instance.destroyInstance(null);

        self.allocator.destroy(self.device.wrapper);
        self.allocator.destroy(self.instance.wrapper);
        vulkan_lib.close();
        self.* = undefined;
    }

    fn createInstance(instance_handle: vk.Instance, instance_dispatch: *InstanceDispatch) Instance {
        return Instance.init(instance_handle, instance_dispatch);
    }

    fn createInstanceHandle(base_dispatch: BaseDispatch) !vk.Instance {
        // Create a Vulkan instance
        const application_info = vk.ApplicationInfo{
            .p_application_name = "Manatee Game",
            .application_version = vk.makeApiVersion(0, 0, 0, 0),
            .engine_version = vk.makeApiVersion(0, 0, 0, 0),
            .api_version = vk.API_VERSION_1_3,
        };

        // TODO: Figure out how to add extensions for non-windows systems
        var instance_extensions = [_][*:0]const u8{
            vk.extensions.khr_surface.name,
            vk.extensions.khr_win_32_surface.name,
        };

        var instance_create_info = vk.InstanceCreateInfo{
            .p_application_info = &application_info,
            .enabled_extension_count = instance_extensions.len,
            .pp_enabled_extension_names = @ptrCast(&instance_extensions),
        };
        const instance_handle = try base_dispatch.createInstance(&instance_create_info, null);
        return instance_handle;
    }

    fn createDevice(device_handle: vk.Device, device_dispatch: *DeviceDispatch) Device {
        return Device.init(device_handle, device_dispatch);
    }

    fn createDeviceHandle(instance: Instance, best_physical_device: DeviceCandidate) !vk.Device {
        const priority = [_]f32{1};
        const device_queue_create_infos = &[_]vk.DeviceQueueCreateInfo{
            .{
                .queue_family_index = best_physical_device.queue_index_graphics,
                .queue_count = 1,
                .p_queue_priorities = &priority,
            },
            .{
                .queue_family_index = best_physical_device.queue_index_present,
                .queue_count = 1,
                .p_queue_priorities = &priority,
            },
        };

        var queue_create_info_count: u32 = 2;
        if (best_physical_device.queue_index_graphics == best_physical_device.queue_index_graphics) {
            queue_create_info_count = 1;
        }

        var device_extensions = [_][*:0]const u8{
            vk.extensions.khr_swapchain.name,
        };

        var device_create_info = vk.DeviceCreateInfo{
            .enabled_extension_count = device_extensions.len,
            .pp_enabled_extension_names = @ptrCast(&device_extensions),
            .queue_create_info_count = queue_create_info_count,
            .p_queue_create_infos = device_queue_create_infos,
        };

        // FINALLY I can create the actual device (Vulkan is so fucking verbose)
        const device_handle = try instance.createDevice(best_physical_device.physical_device, &device_create_info, null);

        return device_handle;
    }

    fn createSurface(instance: Instance, window: *windowing.Window) !vk.SurfaceKHR {
        const surface_create_info = vk.Win32SurfaceCreateInfoKHR{
            .hinstance = @ptrCast(window.hInstance),
            .hwnd = @ptrCast(window.hwnd),
        };
        const surface = try instance.createWin32SurfaceKHR(&surface_create_info, null);
        return surface;
    }

    // TODO:  Document this, this was a bitch to figure out
    fn getInstanceProcAddress(_: vk.Instance, name_ptr: [*:0]const u8) vk.PfnVoidFunction {
        const name = std.mem.span(name_ptr);
        return vulkan_lib.lookup(vk.PfnVoidFunction, name) orelse null;
    }

    fn pickPhysicalDevices(instance: Instance, surface: vk.SurfaceKHR, allocator: std.mem.Allocator) !DeviceCandidate {
        // Get all physical devices
        var physical_devices = try instance.enumeratePhysicalDevicesAlloc(allocator);

        // If the user somehow doesn't have any physical devices, uhh, crash I guess
        if (physical_devices.len == 0) {
            return error.NoPhysicalDevices;
        }

        // One step closer! Now we need to iterate over all of these devices and give them a score
        var best_physical_device: ?vk.PhysicalDevice = null;
        var best_physical_device_score: u32 = 0;

        // Time to iterate and pick that lucky device!
        for (physical_devices[0..physical_devices.len]) |physical_device| {
            const physical_properties = instance.getPhysicalDeviceProperties(physical_device);
            const features = instance.getPhysicalDeviceFeatures(physical_device);

            // We'll start things off low with a score of 0
            var current_score: u32 = 0;

            // We'll then add the maximum screen resolution these devices can render to the score
            current_score += physical_properties.limits.max_image_dimension_2d;

            // Discrete GPUs are often the most powerful (think a physical graphics card that you
            // slot into the ol PCIe port), so we'll make sure to give them bonus points
            if (physical_properties.device_type == .discrete_gpu) {
                current_score += 1000;
            }

            // If devices can't use geometry shaders we simply don't fuck with them, we'll reset
            // the scores of those devices to 0 and hope we find something better
            if (features.geometry_shader == 0) {
                current_score = 0;
            }

            // And now we set the device if its score is greater than the previous one (or it's
            // the first, and potentially only device in the list)
            if (best_physical_device == null or current_score > best_physical_device_score) {
                best_physical_device = physical_device;
                best_physical_device_score = current_score;
            }
        }

        // If the user has a non-zero number of devices and this is somehow still null, time to
        // panic and crash everything!
        if (best_physical_device == null) {
            return error.Unknown;
        }

        // Alrighty, we have the best possible Physical device, yay! We're unfortunately not done
        // iterating over device related things yet. We now need to iterate over the device's
        // queues so we can grab two queue indexes: one for rendering actual graphics, and one for
        // rendering frames to a window (read: surface)

        const queue_family_properties = try instance.getPhysicalDeviceQueueFamilyPropertiesAlloc(best_physical_device.?, allocator);

        // Now remember, we need to fetch two different (or potentially the same, but with separate
        // variables) queue indexes here, let's set up our variables
        var queue_index_graphics: ?u32 = null;
        var queue_index_present: ?u32 = null;

        // Welcome back to the iteration station baby!
        for (queue_family_properties, 0..) |properties, index| {
            // Vulkan wants a u32, yet Zig array indexes are usize, let's cast!
            const family_index: u32 = @intCast(index);

            // If we haven't already picked a graphics index and this one is suitable, set it!
            if (queue_index_graphics == null and properties.queue_flags.graphics_bit) {
                queue_index_graphics = family_index;
            }

            // If we haven't already picked a present index and this one is suitable, set it!
            if (queue_index_present == null and (try instance.getPhysicalDeviceSurfaceSupportKHR(best_physical_device.?, family_index, surface)) == vk.TRUE) {
                queue_index_present = family_index;
            }
        }

        // That was a long-ass function, FINALLY we can return our device candidate and its queues
        return DeviceCandidate{
            .physical_device = best_physical_device.?,
            .physical_properties = instance.getPhysicalDeviceProperties(best_physical_device.?),
            .queue_index_graphics = queue_index_graphics.?,
            .queue_index_present = queue_index_present.?,
        };
    }
};

pub const Queue = struct {
    queue_handle: vk.Queue,
    queue_family: u32,

    fn init(device: Device, queue_family: u32) Queue {
        return .{
            .queue_handle = device.getDeviceQueue(queue_family, 0),
            .queue_family = queue_family,
        };
    }
};

// This should probably go in the main GPUInstance for the best multi backend experience, but the
// Vulkan Zig example I'm following has it separate so I'll also keep it separate for now
pub const Swapchain = struct {
    extent: vk.Extent2D,
    gpu_instance: *GpuInstance,
    surface_format: vk.SurfaceFormatKHR,
    swapchain: vk.SwapchainKHR,
    swapchain_images: []vk.Image,
    pub fn init(gpu_instance: *GpuInstance, allocator: std.mem.Allocator) !Swapchain {
        // Aight so I'm gonna be honest, all of these values are things I'm grabbing from various
        // Vulkan tutorials, and I have no idea what these individually do. I'll need to do a
        // little research and probably fine-tune these values sooner than later
        const surface_capabilities = try gpu_instance.instance.getPhysicalDeviceSurfaceCapabilitiesKHR(gpu_instance.physical_device, gpu_instance.surface);

        var image_count: u32 = surface_capabilities.min_image_count + 1;
        if (image_count > surface_capabilities.max_image_count) {
            image_count = surface_capabilities.max_image_count;
        }

        // TODO: These values should be based off of the surface's width and height rather than
        // the maximums. I'll need to pass the window width / height somewhere into the GPU context
        // since I can't just pull them from the surface... for some reason
        const image_extent = vk.Extent2D{
            .height = surface_capabilities.max_image_extent.height,
            .width = surface_capabilities.max_image_extent.width,
        };

        const surface_format = vk.SurfaceFormatKHR{
            .format = .b8g8r8a8_srgb,
            .color_space = .srgb_nonlinear_khr,
        };

        const image_usage = vk.ImageUsageFlags{
            .color_attachment_bit = true,
            .transfer_dst_bit = true,
        };

        var sharing_mode: vk.SharingMode = .exclusive;
        if (gpu_instance.queue_graphics.queue_family == gpu_instance.queue_present.queue_family) {
            sharing_mode = .concurrent;
        }

        const qfi = [_]u32{ gpu_instance.queue_graphics.queue_family, gpu_instance.queue_present.queue_family };

        const present_mode = try getPresentMode(gpu_instance, allocator);

        const swapchain_create_info = vk.SwapchainCreateInfoKHR{
            .surface = gpu_instance.surface,
            .min_image_count = image_count,
            .image_format = surface_format.format,
            .image_color_space = surface_format.color_space,
            .image_extent = image_extent,
            .image_array_layers = 1,
            .image_usage = image_usage,
            .image_sharing_mode = sharing_mode,
            .queue_family_index_count = qfi.len,
            .pre_transform = surface_capabilities.current_transform,
            .composite_alpha = .{ .opaque_bit_khr = true },
            .present_mode = present_mode,
            .clipped = vk.TRUE,
        };

        const swapchain = try gpu_instance.device.createSwapchainKHR(&swapchain_create_info, null);

        const swapchain_images = try gpu_instance.device.getSwapchainImagesAllocKHR(swapchain, allocator);
        return Swapchain{
            .extent = image_extent,
            .gpu_instance = gpu_instance,
            .surface_format = surface_format,
            .swapchain = swapchain,
            .swapchain_images = swapchain_images,
        };
    }

    pub fn deinit(self: *Swapchain) void {
        self.gpu_instance.device.destroySwapchainKHR(self.swapchain, null);
        self.* = undefined;
    }

    fn getPresentMode(gpu_instance: *GpuInstance, allocator: std.mem.Allocator) !vk.PresentModeKHR {
        const present_modes = try gpu_instance.instance.getPhysicalDeviceSurfacePresentModesAllocKHR(gpu_instance.physical_device, gpu_instance.surface, allocator);

        const preferred = [_]vk.PresentModeKHR{
            .mailbox_khr,
            .immediate_khr,
        };

        for (preferred) |mode| {
            if (std.mem.indexOfScalar(vk.PresentModeKHR, present_modes, mode) != null) {
                return mode;
            }
        }

        return .fifo_khr;
    }
};
