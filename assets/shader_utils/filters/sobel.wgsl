//todo
@group(0) @binding(0) var rawInput: texture_2d<f32>;
@group(0) @binding(1) var filteredOutput: texture_storage_2d<rgba8unorm,write>;

@compute @workgroup_size(8, 8)
fn sobel(@builtin(global_invocation_id) id: vec3<u32>) {

    let color = abs(
        1 * textureLoad(rawInput, vec2<u32>(id.x - 1, id.y - 1), 0).rgb + 2 * textureLoad(rawInput, vec2<u32>(id.x - 1, id.y + 0), 0).rgb + 1 * textureLoad(rawInput, vec2<u32>(id.x - 1, id.y + 1), 0).rgb - 1 * textureLoad(rawInput, vec2<u32>(id.x + 1, id.y - 1), 0).rgb - 2 * textureLoad(rawInput, vec2<u32>(id.x + 1, id.y + 0), 0).rgb - 1 * textureLoad(rawInput, vec2<u32>(id.x + 1, id.y + 1), 0).rgb
    );

    textureStore(filteredOutput, id.xy, vec4<f32>(color, 1.0));
}
