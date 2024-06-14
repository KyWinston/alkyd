#import bevy_pbr::forward_io::{VertexOutput,FragmentOutput}; 
#import bevy_pbr::pbr_types::{STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT, PbrInput, pbr_input_new};
#import bevy_pbr::pbr_functions as fns;
#import bevy_pbr::prepass_utils::{prepass_depth,prepass_normal};
#import bevy_pbr::mesh_view_bindings::view;
#import bevy_pbr::mesh_functions::{get_model_matrix,mesh_normal_local_to_world,mesh_position_local_to_world, mesh_position_local_to_clip};
#import bevy_pbr::mesh_bindings::mesh;
#import utils::{apply_hue,noise2, rand22};

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
    let  I = normalize(in.world_position.xyz - view.world_position);
    let fresnel: f32 = 2.0 * pow(1.0 + dot(I, in.world_normal), 3.0);
    let depth = prepass_depth(in.position, 0u);
    let ignore_detail = fresnel > material.detail_cutoff;

    pbr_input.material.perceptual_roughness = material.roughness;
    pbr_input.material.metallic = material.metallic;
    var grunge_tex: vec4<f32>;
    var grunge_tex_normal: vec4<f32>;
    if ignore_detail {
        grunge_tex_normal = material.diffuse_color * 0.8;
        grunge_tex = material.diffuse_color * 0.8;
    } else {
        grunge_tex_normal = textureSampleBias(brush_handle_normal, normal_sampler, in.uv, view.mip_bias);
        grunge_tex = textureSampleBias(brush_handle, s, in.uv * material.scale.z, view.mip_bias);
    }
    let grunge_normal_distort = mix(vec3(noise2(in.uv * material.distort)), grunge_tex.rgb, material.influence).xy;
    let vsample = mix(in.uv / material.scale.xy, grunge_normal_distort, 0.5);
    let voronoi_base = select(material.diffuse_color.rrr * 0.5, voronoise(vsample), !ignore_detail);
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
        view.mip_bias
    );
    #else
    pbr_input.N = normalize(pbr_input.world_normal)
    #endif
    pbr_input.material.base_color = vec4<f32>(apply_hue(material.diffuse_color.rgb, voronoi_base.y * material.color_varience) * pow(1.0 - voronoi_base.x, 2.0) * pow(vec3<f32>(isBorder), vec3(10.0)), 1.0);
    // pbr_input.material.base_color = vec4<f32>(apply_hue(prepass_normal(in.position,0u).rgb,voronoi_base.y),1.0);
    pbr_input.V = fns::calculate_view(in.world_position, pbr_input.is_orthographic);

    return fns::apply_pbr_lighting(pbr_input);
}

fn voronoise(p: vec2<f32>) -> vec3<f32> {
    var md = 10.0;
    var med = 10.0;
    var tcc: vec2<f32>;
    var cc: vec2<f32>;
    let n = floor(p);
    var cell_id = 0.0;

    for (var x = -1; x <= 1; x++) {
        for (var y = -1; y <= 1; y++) {
            let g = n + vec2(f32(x), f32(y));
            let cache = voro_cache[i32(g.x) + (i32(g.y) * 10)];
            let o = g + cache.xy;
            let r = o - p;
            let d = length(r);

            if d < md {
                md = d;
                cc = g;
                tcc = r;
                cell_id = cache.z;
                if d < cache.w{
                    break;
                }
            }
        }
    }
    for (var z = -1; z <= 1; z++) {
        for (var w = -1; w <= 1; w++) {

            let g = n + vec2(f32(z), f32(w));
            let cache = voro_cache[i32(g.x) + (i32(g.y) * 10)];
            let o = g + cache.xy;
            let r = o - p;
            let dcc = abs(cc - g);
            if !(dcc.x + dcc.y < 0.05) {
                let tc = (tcc + r) * 0.5;
                let cd = normalize(r - tcc);
                let ed = dot(tc, cd);
                med = min(med, ed);
            }
        }
    }

    return vec3<f32>(md, cell_id, med);
}