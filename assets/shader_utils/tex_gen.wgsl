#import utils::{noise2,grid_texture};
#import noise_gen::{FBN};
#import bevy_pbr::mesh_view_bindings::{globals,view};


@group(0) @binding(0)
var<storage> slot_a_in: array<f32>;
@group(0) @binding(1)
var<storage, read_write> slot_a_out: array<f32>;


@compute @workgroup_size(10)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec3<i32>(i32(invocation_id.x), i32(invocation_id.y), i32(invocation_id.z));
    var uv = vec2<f32>(f32(location.x) * f32(location.z) / 1000.0, f32(location.y) * f32(location.z) / 1000.0) * 2.0 - 1.0;
    slot_a_out[location.x * location.z + location.x * location.y * location.z] = slot_a_in[location.x * location.z + location.x * location.y * location.z] + grid_texture(uv * 10.0, 0.9) * grid_texture(vec2(uv + (FBN(uv.xyxy) * 0.05) - vec2(1.36)) * 10.0, 0.8);
}


