
@group(0) @binding(2)
var<storage,read_write> brush_point: array<vec4<f32>>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
  
}
