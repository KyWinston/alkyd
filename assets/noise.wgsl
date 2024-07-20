#import utils::{noise2};

@group(0) @binding(0)
var input: texture_storage_2d<r32float, read>; 

@group(0) @binding(1)
var output: texture_storage_2d<r32float, write>; 

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let uv = vec2<f32>(f32(location.x) / 1920.0 / 4.0 / 8.0, f32(location.y) / 1080.0 / 4.0 / 8.0);
    let seed = vec2<f32>(noise2(uv), noise2(uv * uv));
    textureStore(output, location, vec4<f32>(seed.x));
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let current_noise = textureLoad(input, location)
    textureStore(output, location, current_noise);
}
