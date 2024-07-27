const std = @import("std");

const ShaderCompileStep = @import("vulkan-zig").ShaderCompileStep;

pub fn build(b: *std.Build) void {
    // Sets up the default settings for the build script
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Creates a Zig module for the Manatee engine that allows Manatee to be imported both into the
    // editor as well as any game projects that use it
    const engine_module = b.addModule("manatee", .{
        .root_source_file = b.path("src/engine/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    // Creates the executable for the Manatee Editor
    const editor_exe = b.addExecutable(.{
        .name = "manatee-editor",
        .root_source_file = b.path("src/editor/main.zig"),
        .target = target,
        .optimize = optimize,
    });
    b.installArtifact(editor_exe);

    // Register OS-Specific Dependencies (Why the fuck is doing this in Zig so hard???)
    switch (target.result.os.tag) {
        .macos => blk: {
            // TODO: Figure out which one of these libraries to use
            // const objc = b.dependency("zig-objc", .{});
            // engine_module.addImport("objc", objc.module("objc"));
            // const objc = b.dependency("mach-objc", .{});
            // engine_module.addImport("objc", objc.module("mach-objc"));
            break :blk;
        },
        .windows => blk: {
            const win32 = b.dependency("zigwin32", .{});
            engine_module.addImport("win32", win32.module("zigwin32"));

            // vulkan-zig requires
            const registry = b.dependency("vulkan-headers", .{}).path("registry/vk.xml");
            const vk_gen = b.dependency("vulkan-zig", .{}).artifact("vulkan-zig-generator");

            const vk_generate_cmd = b.addRunArtifact(vk_gen);
            vk_generate_cmd.addFileArg(registry);
            const vulkan_zig = b.addModule("vulkan-zig", .{
                .root_source_file = vk_generate_cmd.addOutputFileArg("vk.zig"),
            });
            engine_module.addImport("vulkan", vulkan_zig);

            // TODO: How the FUCK do I make shaders work for devs using the engine
            const shaders = ShaderCompileStep.create(
                b,
                &[_][]const u8{ "glslc", "--target-env=vulkan1.2" },
                "-o",
            );
            shaders.add("triangle_vert", "shaders/triangle.vert", .{});
            shaders.add("triangle_frag", "shaders/triangle.frag", .{});
            engine_module.addImport("shaders", shaders.getModule());

            // For some reason ZLS autocomplete stops working when editor_exe doesn't have
            // engine_module's imports also added to it. There's probably a way to fix this but I
            // have no idea how, these should probably be stripped before a prod build
            editor_exe.root_module.addImport("vulkan", vulkan_zig);
            editor_exe.root_module.addImport("win32", win32.module("zigwin32"));
            break :blk;
        },
        else => {},
    }

    // Register Engine as a Dependency for Editor allowing it to be imported
    editor_exe.root_module.addImport("manatee", engine_module);

    // Allows us to build and run run the Manatee Editor by running `zig build run`
    const run_cmd = b.addRunArtifact(editor_exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    const run_step = b.step("run", "Build and run the Manatee Editor");
    run_step.dependOn(&run_cmd.step);

    // Sets up unit tests and allows us to run them with `zig build test`
    const unit_tests = b.addTest(.{
        .root_source_file = b.path("src/engine.zig"),
        .target = target,
        .optimize = optimize,
    });
    const test_cmd = b.addRunArtifact(unit_tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&test_cmd.step);

    // Auto-generates API documentation based off of code comments
    const install_docs = b.addInstallDirectory(.{
        .source_dir = editor_exe.getEmittedDocs(),
        .install_dir = .prefix,
        .install_subdir = "docs",
    });
    const docs_step = b.step("docs", "Generate lib documentation");
    docs_step.dependOn(&install_docs.step);
}
