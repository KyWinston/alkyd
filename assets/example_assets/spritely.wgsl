#import bevy_pbr::{
    forward_io::VertexOutput,
    mesh_view_bindings::{view,lights},
    pbr_types::{STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT, PbrInput, pbr_input_new},
    pbr_functions as fns,
}

#import sprite::{read_spritesheet_rotation}
#import utils::{hsv2rgb}

struct Spritely {
    sheet_dimension_x: u32,
    sheet_dimension_y: u32,
    viewing_directions: u32,
    viewing_angle: vec3<f32>,
    player_angle: vec2<f32>,
    current_frame: u32,
    frame_start: vec2<f32>,
    animation_length: u32,
}

@group(2) @binding(0) var<uniform> material:Spritely;
@group(2) @binding(1) var sprite_sheet: texture_2d<f32>;
@group(2) @binding(2) var s: sampler;
@group(2) @binding(3) var uv: texture_2d<f32>;
@group(2) @binding(4) var uv_sampler: sampler;
@group(2) @binding(5) var normals: texture_2d<f32>;
@group(2) @binding(6) var s_normals: sampler;
@group(2) @binding(7) var occlusion: texture_2d<f32>;
@group(2) @binding(8) var s_occ: sampler;
@group(2) @binding(9) var volume: texture_2d<f32>;
@group(2) @binding(10) var s_vol: sampler;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool
) -> @location(0) vec4<f32> {
    var pbr_input: PbrInput = pbr_input_new();

    var sprite_rotation:vec2<f32> = read_spritesheet_rotation(material.player_angle, material.viewing_angle, in.uv.x);
    var angle = sprite_rotation.y;
    let y_offset: f32 = (material.frame_start.y + f32(material.current_frame)) / f32(material.sheet_dimension_y);

    sprite_rotation.x /= f32(material.sheet_dimension_x);

    var offset: f32 = material.frame_start.x + (f32(step(45.0, abs(angle)) + step(90.0, abs(angle)) + step(135.0, abs(angle))));
    var uv_offset: u32 = 0u;

    let frame = vec2<f32>(sprite_rotation.x + f32(offset) / f32(material.sheet_dimension_x), in.uv.y / (-1.0 * f32(material.sheet_dimension_y)) + y_offset);
    let sprite = textureSample(sprite_sheet, s, frame);
    let color_map = vec4(vec3(textureSample(uv, uv_sampler, sprite.rg).rgb), sprite.a);
    let vol = textureSample(volume, s_vol, frame);

    pbr_input.frag_coord = in.position;
    pbr_input.world_position = in.world_position * vol;
    let ao = normalize(lights.ambient_color.rgb) * textureSample(occlusion, s_occ, frame).rgb;

    pbr_input.material.base_color = vec4(vec3(color_map.rgb), color_map.a);

    pbr_input.V = fns::calculate_view(pbr_input.frag_coord, false);
    let diffuse = fns::apply_pbr_lighting(pbr_input);
    return vec4f(vec3f(diffuse.rgb * ao), diffuse.a);
}
