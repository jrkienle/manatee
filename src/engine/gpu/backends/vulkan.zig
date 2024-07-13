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
    vk.extensions.khr_win_32_surface,
};

// These structs are actually just typings used to generate the actual instances in Instance.init()
const BaseDispatch = vk.BaseWrapper(apis);
const InstanceDispatch = vk.InstanceWrapper(apis);
const DeviceDispatch = vk.DeviceWrapper(apis);

pub const Instance = struct {
    base_dispatch: BaseDispatch,
    instance: vk.Instance,
    instance_dispatch: InstanceDispatch,
    device: vk.Device,
    device_dispatch: DeviceDispatch,

    pub fn init() !Instance {
        // Before we go any further, let's load the Vulkan lib (this will crash if not available)
        vulkan_lib = try std.DynLib.open(switch (builtin.target.os.tag) {
            .windows => "vulkan-1.dll",
            // TODO: I want to add Vulkan support to Mac and Linux in the future, but the MVP of
            // this engine will likely be Windows-only
            // .linux => "libvulkan.so.1",
            // .macos => "libvulkan.1.dylib",
            else => @compileError(std.fmt.comptimePrint("Unsupported OS: {}", .{builtin.os.tag})),
        });

        // Create a proxy for the Vulkan lib
        const base_dispatch = try BaseDispatch.load(getInstanceProcAddress);

        // Create the Vulkan instance
        const instance = try createInstance(base_dispatch);

        // Create a proxy for the instance
        const instance_dispatch = try InstanceDispatch.load(instance, base_dispatch.dispatch.vkGetInstanceProcAddr);

        const device = try createDevice(instance_dispatch, instance);

        // Create a proxy for the device
        const device_dispatch = try DeviceDispatch.load(device, instance_dispatch.dispatch.vkGetDeviceProcAddr);

        return Instance{
            .base_dispatch = base_dispatch,
            .instance = instance,
            .instance_dispatch = instance_dispatch,
            .device = device,
            .device_dispatch = device_dispatch,
        };
    }

    pub fn deinit(self: *Instance) void {
        self.device_dispatch.destroyDevice(self.device, null);
        self.instance_dispatch.destroyInstance(self.instance, null);
        vulkan_lib.close();
        self.* = undefined;
    }

    // TODO:  Document this, this was a bitch to figure out
    fn getInstanceProcAddress(_: vk.Instance, name_ptr: [*:0]const u8) vk.PfnVoidFunction {
        const name = std.mem.span(name_ptr);
        return vulkan_lib.lookup(vk.PfnVoidFunction, name) orelse null;
    }

    fn createInstance(base_dispatch: BaseDispatch) !vk.Instance {
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
        const instance = try base_dispatch.createInstance(&instance_create_info, null);
        return instance;
    }

    fn createDevice(instance_dispatch: InstanceDispatch, instance: vk.Instance) !vk.Device {
        var physical_devices_count: u32 = 0;
        _ = try instance_dispatch.enumeratePhysicalDevices(instance, &physical_devices_count, null);

        if (physical_devices_count == 0) {
            return error.NoPhysicalDevices;
        }

        var gpa = std.heap.GeneralPurposeAllocator(.{}){};
        defer _ = gpa.deinit();
        const allocator = gpa.allocator();
        var physical_devices = try allocator.alloc(vk.PhysicalDevice, physical_devices_count);
        defer allocator.free(physical_devices);

        _ = try instance_dispatch.enumeratePhysicalDevices(instance, &physical_devices_count, physical_devices.ptr);

        const DeviceCandidate = struct {
            physical_device: vk.PhysicalDevice,
            physical_properties: vk.PhysicalDeviceProperties,
            queue_family: u32,
        };

        var best_physical_device: ?DeviceCandidate = null;
        var best_physical_device_score: u32 = 0;

        // TODO: Break this shit out into utility functions AND COMMENT THAT SHIT
        for (physical_devices[0..physical_devices_count]) |physical_device| {
            const physical_properties = instance_dispatch.getPhysicalDeviceProperties(physical_device);
            const features = instance_dispatch.getPhysicalDeviceFeatures(physical_device);

            var current_score: u32 = 0;

            if (physical_properties.device_type == .discrete_gpu) {
                current_score += 1000;
            }

            current_score += physical_properties.limits.max_image_dimension_2d;

            if (best_physical_device == null or current_score > best_physical_device_score) {
                var queue_family: ?u32 = null;
                var queue_family_property_count: u32 = 0;
                _ = instance_dispatch.getPhysicalDeviceQueueFamilyProperties(physical_device, &queue_family_property_count, null);
                const queue_family_properties = try allocator.alloc(vk.QueueFamilyProperties, queue_family_property_count);
                defer allocator.free(queue_family_properties);
                _ = instance_dispatch.getPhysicalDeviceQueueFamilyProperties(physical_device, &queue_family_property_count, queue_family_properties.ptr);
                for (queue_family_properties, 0..) |family, i| {
                    if (family.queue_flags.compute_bit) {
                        queue_family = @intCast(i);
                    }
                }

                best_physical_device = DeviceCandidate{
                    .physical_device = physical_device,
                    .physical_properties = physical_properties,
                    .queue_family = queue_family.?,
                };
                best_physical_device_score = current_score;
            }

            // If devices can't use geometry shaders, Vulkan can't use them at all
            if (features.geometry_shader == 0) {
                current_score = 0;
            }
        }

        if (best_physical_device == null) {
            return error.NoPhysicalDevices;
        }

        const device_queue_create_infos = &[_]vk.DeviceQueueCreateInfo{.{
            .queue_family_index = best_physical_device.?.queue_family,
            .queue_count = 1,
            .p_queue_priorities = &[_]f32{1.0},
        }};

        var device_create_info = vk.DeviceCreateInfo{
            // TODO: Do I need device extensions?
            // .enabled_extension_count = device_extensions.len,
            // .pp_enabled_extension_names = device_extensions_arr.slice().ptr,
            .queue_create_info_count = 1,
            .p_queue_create_infos = device_queue_create_infos,
        };

        // FINALLY I can create the actual device (Vulkan is so fucking verbose)
        const device = try instance_dispatch.createDevice(best_physical_device.?.physical_device, &device_create_info, null);

        return device;
    }
};

pub const Surface = struct {
    instance: *Instance,
    surface: vk.SurfaceKHR,

    pub fn init(instance: *Instance, window: windowing.Window) !Surface {
        const surface_create_info = vk.Win32SurfaceCreateInfoKHR{
            .hinstance = @ptrCast(window.hInstance),
            .hwnd = @ptrCast(window.hwnd),
        };
        const surface = try instance.instance_dispatch.createWin32SurfaceKHR(instance.instance, &surface_create_info, null);

        return Surface{
            .instance = instance,
            .surface = surface,
        };
    }

    pub fn deinit(self: *Surface) void {
        self.instance.instance_dispatch.destroySurfaceKHR(self.instance.instance, self.surface, null);
        self.* = undefined;
    }
};
