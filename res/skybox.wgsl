// Vertex shader

struct VertexInput {
    @location(0)
    position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position)
    clip_position: vec4<f32>,
}

struct Data {
    proj_mat: mat4x4<f32>,
    proj_mat_inv: mat4x4<f32>,
    view_mat: mat4x4<f32>,
    view_pos: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> data: Data;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position =  vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@group(1) @binding(0)
var cubeTexture: texture_cube<f32>;

@group(1) @binding(1)
var cubeSampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(cubeTexture, cubeSampler, in.clip_position.xyz);
}
