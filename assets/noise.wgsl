
@group(0) @binding(0)
var<uniform> cell_size: f32;

@group(0) @binding(1)
var<storage,read_write> centroids_out: array<vec4<f32>>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let uv = vec2<f32>(f32(location.x) / 10.0, f32(location.y) / 10.0);
    let seed = hash_three(uv);
    centroids_out[location.x + (location.y * 10)] = vec4<f32>(seed, cell_size);
}

fn hash_three(p: vec2<f32>) -> vec3<f32> {
    let q: vec3<f32>= vec3(dot(p, vec2(127.1, 311.7)),
            dot(p, vec2(269.5, 183.3)),
            dot(p, vec2(419.2, 371.9)));
    return fract(sin(q) * 43758.5453);
}

