
@group(0) @binding(0)
var<storage,read_write> surface: array<array<vec3<f32>,3>>;

@group(0) @binding(1)
var<storage,read_write> points: array<vec3<f32>,10000>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
}


