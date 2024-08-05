// Vertex shader

struct Matrices {
    world: mat4x4<f32>,
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> matrices: Matrices;

@group(1) @binding(0)
var<uniform> color: vec3<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = matrices.view_proj * matrices.world * vec4<f32>(in.position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(color, 1.0);
}
