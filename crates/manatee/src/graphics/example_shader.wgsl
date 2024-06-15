// This comes from the learn wgpu tutorial that's all over the internet right now. I don't know how
// shaders work, I don't know why I need them for simple rendering, I don't know anything and I'm
// trying to build a game engine lmao. Regardless, this is needed to make rendering work I guess,
// so I'll hopefully understand the wgsl language better some day soon

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

// This is a vertex shader, and I guess it's required?'
@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    // Wtf is all of this math doing
    // let x = f32(1 - i32(in_vertex_index)) * 0.5;
    // let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    // The 4th element in clip position is scale relative to the distance of the projection. I
    // think it should usually be left at 1.0?
    // TODO: Figure out if the position can be engine configurable
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.color = model.color;
    return out;
}

// Fragment shaders handle taking whatever was created in the Vertex shader and actually rendering
// it as pixels on the screen with optional transformations such as color
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // This is the accumulated surface color in 0-1 RGBA
    return vec4<f32>(in.color, 1.0);
}
