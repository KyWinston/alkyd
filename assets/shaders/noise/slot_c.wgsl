@group(0) @binding(0)
var buffer_a: texture_storage_2d<rgba32float, read_write>;

@group(0) @binding(1)
var buffer_b: texture_storage_2d<rgba32float, read_write>;

@group(0) @binding(2)
var buffer_c: texture_storage_2d<rgba32float, read_write>;

@group(0) @binding(3)
var buffer_d: texture_storage_2d<rgba32float, read_write>;



@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
}