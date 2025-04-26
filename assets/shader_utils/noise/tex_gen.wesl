#import noise_gen::FBN;

@group(0) @binding(0) var first_half: texture_storage_2d<r32float, read_write>;
@group(0) @binding(1) var second_half: texture_storage_2d<r32float, read_write>;

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec3<i32>(i32(invocation_id.x), i32(invocation_id.y), i32(invocation_id.z));
    let uv = vec3f(f32(location.x) / 1920.0, f32(location.y) / 1920.0, f32(location.z) / 1920.0);
    let color: f32 = FBN(vec4<f32>(uv.x, uv.y, uv.z, 1.0));

    textureStore(first_half, location.xy, vec4f(color, 0.0, 0.0, 1.0));
    textureStore(second_half, location.xz, vec4f(color, 0.0, 0.0, 1.0));
}

