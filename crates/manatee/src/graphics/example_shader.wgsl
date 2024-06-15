// This comes from the learn wgpu tutorial that's all over the internet right now. I don't know how
// shaders work, I don't know why I need them for simple rendering, I don't know anything and I'm
// trying to build a game engine lmao. Regardless, this is needed to make rendering work I guess,
// so I'll hopefully understand the wgsl language better some day soon

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

// This is a vertex shader, and I guess it's required?'
@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    // Wtf is all of this math doing
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

// Fragment shaders are also required?
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Where did these numbers come from
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}
