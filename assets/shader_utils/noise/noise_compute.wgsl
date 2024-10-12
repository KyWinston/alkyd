#import utils::{noise2};

@group(0) @binding(0)
var<storage,read_write> centroids: array<vec4<f32>>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let uv = vec2<f32>(f32(location.x), f32(location.y));
    let seed = vec2<f32>(noise2(uv), noise2(uv * uv));
    centroids[location.x + location.y * 100] = vec4<f32>(seed, noise2(vec2<f32>(f32(location.x), f32(location.y))), 0.1);
}


