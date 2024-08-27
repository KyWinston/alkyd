#import bevy_pbr::forward_io::{VertexOutput,FragmentOutput};
#import bevy::core_pipeline::fullscreen_vertex_shader::FullScreenVertexOutput;
#import utils::{noise2,grid_texture};
#import noise_gen::{FBN};

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
// @group(0) @binding(1) var slot_a: texture_storage_2d<r32float,read_write>;
// @group(1) @binding(1) var s: sampler;

@fragment
fn fragment(
    in: FullScreenVertexOutput,
) -> @location(0) vec4<f32> {
    return vec4(vec3(grid_texture(uv * 10.0, 0.9) * grid_texture(vec2(uv + (FBN(uv.xyxy) * 0.05) - vec2(1.36)) * 10.0, 0.8)), 1.0);
}

