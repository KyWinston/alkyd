#import utils::{noise2};

@group(0) @binding(0)
var buffer_a: texture_storage_2d<rgba32float, read_write>;

@group(0) @binding(1)
var buffer_b: texture_storage_2d<rgba32float, read_write>;

@group(0) @binding(2)
var buffer_c: texture_storage_2d<rgba32float, read_write>;

@group(0) @binding(3)
var buffer_d: texture_storage_2d<rgba32float, read_write>;

@group(0) @binding(4)
var texture: texture_storage_2d<rgba32float, read_write>;

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let uv = vec2<f32>(f32(location.x) / 2048.0 / 4.0 / 8.0, f32(location.y) / 2048.0 / 4.0, 8.0);
    let seed = vec2<f32>(noise2(uv), noise2(uv * uv));
    var O: vec4<f32> = textureLoad(buffer_a, location);
    textureStore(texture, location, vec4<f32>(seed.x));
}


