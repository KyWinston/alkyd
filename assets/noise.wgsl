#import utils::{noise2};

@group(0) @binding(0)
var<uniform> cell_size: f32;

@group(0) @binding(1)
var<storage,read_write> centroids_out: array<vec4<f32>>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let uv = vec2<f32>(f32(location.x), f32(location.y));
    // let min_dist = 0.3;
    let cell = cell_size;
    // let current = centroids_out[location.x + location.y * 20 + location.z * 200].xyz;
    // let neighbor = centroids_out[(location.x - 1) + (location.y - 1) * 20 + (location.z - 1) * 200].xyz;
    let seed = vec2<f32>(noise2(uv), noise2(uv * uv));
    centroids_out[location.x + location.y * 10] = vec4<f32>(seed, noise2(vec2<f32>(f32(location.x), f32(location.y))), cell_size);
}