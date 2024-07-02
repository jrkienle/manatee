const std = @import("std");

pub fn build(b: *std.Build) void {
    // Sets up the default settings for the build script
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Creates a Manatee static library for the Manatee engine. This allows Manatee to be used with
    // other languages as long as they support loading static libs
    const engine = b.addStaticLibrary(.{
        .name = "manatee",
        .root_source_file = b.path("src/engine.zig"),
        .target = target,
        .optimize = optimize,
    });
    b.installArtifact(engine);

    // Creates the executable for the Manatee Editor
    const editor = b.addExecutable(.{
        .name = "manatee-editor",
        .root_source_file = b.path("src/editor.zig"),
        .target = target,
        .optimize = optimize,
    });
    b.installArtifact(editor);

    // Allows us to build and run run the Manatee Editor by running `zig build run`
    const run_cmd = b.addRunArtifact(editor);
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
        .source_dir = engine.getEmittedDocs(),
        .install_dir = .prefix,
        .install_subdir = "docs",
    });
    const docs_step = b.step("docs", "Generate lib documentation");
    docs_step.dependOn(&install_docs.step);
}
