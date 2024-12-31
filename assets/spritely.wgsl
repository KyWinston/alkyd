#import bevy_pbr::{
    forward_io::VertexOutput,
    mesh_view_bindings::{view,lights},
    pbr_types::{STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT, PbrInput, pbr_input_new},
    pbr_functions as fns,
}

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

    let dir1 = material.player_angle;
    let dir2 = material.viewing_angle.xz;
    let dot = dir1.x * dir2.x + dir1.y * dir2.y;
    let det = dir1.x * dir2.y - dir1.y * dir2.x;

    let angle = degrees(atan2(dot, det));
    var offset: f32 = material.frame_start.x + (f32(step(45.0, abs(angle)) + step(90.0, abs(angle)) + step(135.0, abs(angle))));
    var anim_idx = in.uv.x;
    let backface = abs(angle) >= -180.0 && abs(angle) < -170.0;
    let mirror = angle > 44.0 && angle < 170.0;

    var uv_offset: u32 = 0u;

    if backface {
        anim_idx = 0.0;
    }

    if backface || mirror {
        anim_idx = 1.0 - anim_idx;
    }

    let y_offset: f32 = (material.frame_start.y + f32(material.current_frame)) / f32(material.sheet_dimension_y);
    anim_idx /= f32(material.sheet_dimension_x);

    let frame = vec2<f32>(anim_idx + f32(offset) / f32(material.sheet_dimension_x), in.uv.y / (-1.0 * f32(material.sheet_dimension_y)) + y_offset);
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
