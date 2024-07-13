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
const DeviceCandidate = struct {
    physical_device: vk.PhysicalDevice,
    physical_properties: vk.PhysicalDeviceProperties,
    queue_index_graphics: u32,
    queue_index_present: u32,
};

pub const GpuInstance = struct {
    base_dispatch: BaseDispatch,
    instance: vk.Instance,
    instance_dispatch: InstanceDispatch,
    device: vk.Device,
    device_dispatch: DeviceDispatch,
    queue_graphics: vk.Queue,
    queue_present: vk.Queue,
    surface: vk.SurfaceKHR,

    pub fn init(window: *windowing.Window) !GpuInstance {
        vulkan_lib = try std.DynLib.open(switch (builtin.target.os.tag) {
            .windows => "vulkan-1.dll",
            // TODO: I want to add Vulkan support to Mac and Linux in the future, but the MVP of
            // this engine will likely be Windows-only
            // .linux => "libvulkan.so.1",
            // .macos => "libvulkan.1.dylib",
            else => @compileError(std.fmt.comptimePrint("Unsupported OS: {}", .{builtin.os.tag})),
        });
        const base_dispatch = try BaseDispatch.load(getInstanceProcAddress);

        const instance = try createInstance(base_dispatch);
        const instance_dispatch = try InstanceDispatch.load(instance, base_dispatch.dispatch.vkGetInstanceProcAddr);

        const surface = try createSurface(instance_dispatch, instance, window);

        const best_physical_device = try pickPhysicalDevices(instance_dispatch, instance, surface);
        const device = try createDevice(instance_dispatch, best_physical_device);
        const device_dispatch = try DeviceDispatch.load(device, instance_dispatch.dispatch.vkGetDeviceProcAddr);

        const queue_graphics = device_dispatch.getDeviceQueue(device, best_physical_device.queue_index_graphics, 0);
        const queue_present = device_dispatch.getDeviceQueue(device, best_physical_device.queue_index_present, 0);

        return GpuInstance{
            .base_dispatch = base_dispatch,
            .instance = instance,
            .instance_dispatch = instance_dispatch,
            .device = device,
            .device_dispatch = device_dispatch,
            .queue_graphics = queue_graphics,
            .queue_present = queue_present,
            .surface = surface,
        };
    }

    pub fn deinit(self: *GpuInstance) void {
        self.device_dispatch.destroyDevice(self.device, null);
        self.instance_dispatch.destroySurfaceKHR(self.instance, self.surface, null);
        self.instance_dispatch.destroyInstance(self.instance, null);
        vulkan_lib.close();
        self.* = undefined;
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

    fn createDevice(instance_dispatch: InstanceDispatch, best_physical_device: DeviceCandidate) !vk.Device {
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

        var device_create_info = vk.DeviceCreateInfo{
            // TODO: Do I need device extensions?
            // .enabled_extension_count = device_extensions.len,
            // .pp_enabled_extension_names = device_extensions_arr.slice().ptr,
            .queue_create_info_count = queue_create_info_count,
            .p_queue_create_infos = device_queue_create_infos,
        };

        // FINALLY I can create the actual device (Vulkan is so fucking verbose)
        const device = try instance_dispatch.createDevice(best_physical_device.physical_device, &device_create_info, null);

        return device;
    }

    fn createSurface(instance_dispatch: InstanceDispatch, instance: vk.Instance, window: *windowing.Window) !vk.SurfaceKHR {
        const surface_create_info = vk.Win32SurfaceCreateInfoKHR{
            .hinstance = @ptrCast(window.hInstance),
            .hwnd = @ptrCast(window.hwnd),
        };
        const surface = try instance_dispatch.createWin32SurfaceKHR(instance, &surface_create_info, null);
        return surface;
    }

    // TODO:  Document this, this was a bitch to figure out
    fn getInstanceProcAddress(_: vk.Instance, name_ptr: [*:0]const u8) vk.PfnVoidFunction {
        const name = std.mem.span(name_ptr);
        return vulkan_lib.lookup(vk.PfnVoidFunction, name) orelse null;
    }

    fn pickPhysicalDevices(instance_dispatch: InstanceDispatch, instance: vk.Instance, surface: vk.SurfaceKHR) !DeviceCandidate {
        // In order to select the best device to render graphics with, we first need to iterate
        // over all of the user's physical devices and see what we have available. This is really
        // unintuitive both in Vulkan and Zig. Effectively, we need to do the following:
        // 1. Count how many physical devices the machine has
        // 2. Use an allocator to create an empty array to store references to those devices
        // 3. Iterate over the devices and give each of them a score
        // 4. Return the device with the highest score

        // The first step is to create an allocator, for this one I'll be using a general purpose
        // allocator because that's what every piece of example code I've seen uses
        var gpa = std.heap.GeneralPurposeAllocator(.{}){};
        defer _ = gpa.deinit();
        const allocator = gpa.allocator();

        // Now it's time to count the machine's physical devices
        var physical_devices_count: u32 = 0;
        _ = try instance_dispatch.enumeratePhysicalDevices(instance, &physical_devices_count, null);

        // If the user somehow doesn't have any physical devices, uhh, crash I guess
        if (physical_devices_count == 0) {
            return error.NoPhysicalDevices;
        }

        // Time to allocate an empty array for these devices and then, using its pointer, fill it
        var physical_devices = try allocator.alloc(vk.PhysicalDevice, physical_devices_count);
        defer allocator.free(physical_devices);
        _ = try instance_dispatch.enumeratePhysicalDevices(instance, &physical_devices_count, physical_devices.ptr);

        // One step closer! Now we need to iterate over all of these devices and give them a score
        var best_physical_device: ?vk.PhysicalDevice = null;
        var best_physical_device_score: u32 = 0;

        // Time to iterate and pick that lucky device!
        for (physical_devices[0..physical_devices_count]) |physical_device| {
            const physical_properties = instance_dispatch.getPhysicalDeviceProperties(physical_device);
            const features = instance_dispatch.getPhysicalDeviceFeatures(physical_device);

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
        // rendering frames to a window (read: surface). These steps will look shockingly familiar
        // to picking the best device, and yes, they start with an allocator. Luckily we slapped a
        // defer before destroying our previous allocator, so we can reuse that one

        var queue_family_property_count: u32 = 0;
        instance_dispatch.getPhysicalDeviceQueueFamilyProperties(best_physical_device.?, &queue_family_property_count, null);

        // If the devoce somehow doesn't have any queue families, uhh, crash I guess. I probably
        // need to add some better error handling into here, there's a lot of happy path
        if (physical_devices_count == 0) {
            return error.NoQueueFamilies;
        }

        // Yay, more array allocation (I'm scared of memory yet using Zig)
        const queue_family_properties = try allocator.alloc(vk.QueueFamilyProperties, queue_family_property_count);
        defer allocator.free(queue_family_properties);
        instance_dispatch.getPhysicalDeviceQueueFamilyProperties(best_physical_device.?, &queue_family_property_count, queue_family_properties.ptr);

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
            if (queue_index_present == null and (try instance_dispatch.getPhysicalDeviceSurfaceSupportKHR(best_physical_device.?, family_index, surface)) == vk.TRUE) {
                queue_index_present = family_index;
            }
        }

        // That was a long-ass function, FINALLY we can return our device candidate and its queues
        return DeviceCandidate{
            .physical_device = best_physical_device.?,
            .physical_properties = instance_dispatch.getPhysicalDeviceProperties(best_physical_device.?),
            .queue_index_graphics = queue_index_graphics.?,
            .queue_index_present = queue_index_present.?,
        };
    }
};
