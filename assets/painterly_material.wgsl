
#import bevy_pbr::forward_io::{VertexOutput,FragmentOutput}; 
#import bevy_pbr::pbr_types::{STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT, PbrInput, pbr_input_new};
#import bevy_pbr::pbr_functions as fns;
#import bevy_pbr::prepass_utils::{prepass_depth,prepass_normal};
#import bevy_pbr::mesh_view_bindings::view;
#import bevy_pbr::mesh_functions::{get_model_matrix,mesh_normal_local_to_world,mesh_position_local_to_world, mesh_position_local_to_clip};
#import bevy_pbr::mesh_bindings::mesh;
#import utils::{apply_hue,noise2, rand22, voronoi};

struct Painterly {
    diffuse_color: vec4<f32>,
    roughness: f32,
    metallic: f32,
    color_varience: f32,
    scale: vec3<f32>,
    distort: f32,
    influence: f32,
    border: f32,
    dist_falloff: f32,
    detail_cutoff: f32
}

@group(2) @binding(0) var<uniform> material:Painterly;
@group(2) @binding(1) var brush_handle: texture_2d<f32>;
@group(2) @binding(2) var s: sampler;
@group(2) @binding(3) var brush_handle_normal: texture_2d<f32>;
@group(2) @binding(4) var normal_sampler: sampler;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    var pbr_input: PbrInput = pbr_input_new();
    let double_sided = (pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u;
    pbr_input.frag_coord = in.position;
    pbr_input.world_position = in.world_position;
    let  I = normalize(in.world_position.xyz - view.world_position);
    let fresnel: f32 = 2.0 * pow(1.0 + dot(I, in.world_normal), 3.0);
    let depth: f32 = prepass_depth(in.position, 0u);
    let ignore_detail = fresnel > material.detail_cutoff + depth * 3.0;

    pbr_input.material.perceptual_roughness = material.roughness;
    pbr_input.material.metallic = material.metallic;
    var grunge_tex: vec4<f32>;
    var grunge_tex_normal: vec4<f32>;
    if ignore_detail {
        grunge_tex_normal = material.diffuse_color * 0.7;
        grunge_tex = material.diffuse_color * 0.7;
    } else {
        grunge_tex_normal = textureSampleBias(brush_handle_normal, normal_sampler, in.uv, view.mip_bias);
        grunge_tex = textureSampleBias(brush_handle, s, in.uv * material.scale.z, view.mip_bias);
    }
    let grunge_normal_distort: vec2<f32> = mix(vec3<f32>(noise2(in.uv * material.distort)), grunge_tex.rgb, material.influence).xy;
    let vsample = mix(in.uv / material.scale.xy, grunge_normal_distort, 0.5);
    let voronoi_base = select(material.diffuse_color.rgb - 0.3, voronoi(vsample, depth), !ignore_detail);
    let valueChange = length(fwidth(vsample)) * material.dist_falloff;
    let isBorder = smoothstep(material.border - valueChange, material.border + valueChange, voronoi_base.z);
    pbr_input.world_normal = fns::prepare_world_normal(
        select(in.world_normal, apply_hue(in.world_normal, voronoi_base.x), !ignore_detail),
        double_sided,
        is_front
    );
   #ifdef VERTEX_TANGENTS
    let Nt = grunge_tex_normal.rgb;
    let TBN = fns::calculate_tbn_mikktspace(pbr_input.world_normal.rgb,
        in.world_tangent);
    pbr_input.N = fns::apply_normal_mapping(
        pbr_input.material.flags,
        TBN,
        double_sided,
        is_front,
        Nt,
    );
    #else
    pbr_input.N = normalize(pbr_input.world_normal);
    #endif
    pbr_input.material.base_color = vec4<f32>(apply_hue(material.diffuse_color.rgb, voronoi_base.y * material.color_varience) * pow(1.0 - voronoi_base.x, 2.0) * pow(vec3<f32>(isBorder), vec3(10.0)) - (valueChange / 50.0), 1.0);
    pbr_input.V = fns::calculate_view(in.world_position, pbr_input.is_orthographic);

    return fns::apply_pbr_lighting(pbr_input);
}

