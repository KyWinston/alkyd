#import bevy_pbr::forward_io::{VertexOutput,FragmentOutput}; 
#import bevy_pbr::pbr_types::{STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT, PbrInput, pbr_input_new};
#import bevy_pbr::pbr_functions as fns;
#import bevy_pbr::prepass_utils::prepass_normal;
#import bevy_pbr::mesh_view_bindings::view;
#import bevy_pbr::mesh_functions::{get_model_matrix,mesh_normal_local_to_world,mesh_position_local_to_world, mesh_position_local_to_clip};
#import bevy_pbr::mesh_bindings::mesh;

struct Painterly {
    diffuse_color: vec4<f32>,
    roughness: f32,
    metallic: f32,
    color_varience: f32,
    scale: f32,
    distort: f32,
    influence: f32,
}

@group(2) @binding(0) var<uniform> material:Painterly;
@group(2) @binding(1) var brush_handle: texture_2d<f32>;
@group(2) @binding(2) var s: sampler;
@group(2) @binding(3) var brush_handle_normal: texture_2d<f32>;
@group(2) @binding(4) var normal_sampler: sampler;
@group(2) @binding(5) var<storage, read_write> voro_cache: array<vec4<f32>>;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    var pbr_input: PbrInput = pbr_input_new();

    let double_sided = (pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u;

    pbr_input.frag_coord = in.position;
    pbr_input.world_position = in.world_position;
    let view_angle = in.world_position.xyz - view.world_position;
    let view_dist = distance(in.world_position.xyz, view.world_position);

    pbr_input.material.perceptual_roughness = material.roughness;
    pbr_input.material.metallic = material.metallic;
    let grunge_tex_normal = textureSample(brush_handle_normal, normal_sampler, in.uv);
    let grunge_tex = textureSample(brush_handle, s, in.uv);
    let grunge_normal_distort = mix(vec3(noise2(in.uv * material.distort)), grunge_tex.rgb, material.influence).xy;

    let voronoi_base = voronoise(mix(in.uv / material.scale, grunge_normal_distort, 0.5));

    pbr_input.world_normal = fns::prepare_world_normal(
        apply_hue(in.world_normal, voronoi_base),
        double_sided,
        is_front
    );

    pbr_input.N = mix(fns::apply_normal_mapping(
        pbr_input.material.flags,
        pbr_input.world_normal,
        double_sided,
        is_front,
        in.uv,
        view.mip_bias
    ), grunge_tex_normal.rgb, 0.5);

    pbr_input.material.base_color = vec4(apply_hue(material.diffuse_color.rgb, voronoi_base * material.color_varience), 1.0);

    pbr_input.V = fns::calculate_view(in.world_position, pbr_input.is_orthographic);
    return fns::apply_pbr_lighting(pbr_input);
}


fn voronoise(p: vec2<f32>) -> f32 {

    var min_dist = 10.0;
    var closest_cell: f32;
    for (var x = -1; x <= 1; x++) {
        for (var y = -1; y <= 1; y++) {
            let cell = floor(p) + vec2(f32(x), f32(y));
            let cell_pos = vec3(vec2(cell), 0.0) + voro_cache[i32(cell.x) + (i32(cell.y) * 10)].xyz;
            let to_cell = cell_pos.xy - p;
            let dist_to_cell = length(to_cell);
            if dist_to_cell < min_dist {
                min_dist = dist_to_cell;
                closest_cell = cell_pos.z;
            }
        }
    }

    return closest_cell;
}

fn apply_hue(col: vec3<f32>, hueAdjust: f32) -> vec3<f32> {
    let k = vec3(0.57735, 0.57735, 0.57735);
    let cosAngle = cos(hueAdjust);
    return col * cosAngle + cross(k, col) * sin(hueAdjust) + k * dot(k, col) * (1.0 - cosAngle);
}

// WTFPL License
fn noise2(n: vec2<f32>) -> f32 {
    let d = vec2f(0., 1.);
    let b = floor(n);
    let f = smoothstep(vec2(0.), vec2(1.), fract(n));
    return mix(mix(rand22(b), rand22(b + d.yx), f.x), mix(rand22(b + d.xy), rand22(b + d.yy), f.x), f.y);
}

fn rand22(n: vec2<f32>) -> f32 { return fract(sin(dot(n, vec2(12.9898, 4.1414))) * 43758.5453); }