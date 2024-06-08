#import bevy_pbr::forward_io::{VertexOutput,FragmentOutput}; 
#import bevy_pbr::pbr_types::{STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT, PbrInput, pbr_input_new};
#import bevy_pbr::pbr_functions as fns;
#import bevy_pbr::prepass_utils::{prepass_depth,prepass_normal};
#import bevy_pbr::mesh_view_bindings::view;
#import bevy_pbr::mesh_functions::{get_model_matrix,mesh_normal_local_to_world,mesh_position_local_to_world, mesh_position_local_to_clip};
#import bevy_pbr::mesh_bindings::mesh;

struct Painterly {
    diffuse_color: vec4<f32>,
    roughness: f32,
    metallic: f32,
    color_varience: f32,
    scale: vec3<f32>,
    distort: f32,
    influence: f32,
    border: f32,
    dist_falloff: f32
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
    let depth = prepass_depth(in.position, 0u);
    pbr_input.material.perceptual_roughness = material.roughness;
    pbr_input.material.metallic = material.metallic;
    let grunge_tex_normal = textureSampleBias(brush_handle_normal, normal_sampler, in.uv, view.mip_bias);
    let grunge_tex = textureSampleBias(brush_handle, s, in.uv * material.scale.z, view.mip_bias);
    let grunge_normal_distort = mix(vec3(noise2(in.uv * material.distort)), grunge_tex.rgb, material.influence).xy;
    let vsample = mix(in.uv / material.scale.xy, grunge_normal_distort, 0.5);
    let voronoi_base = voronoise(vsample, depth);
    let valueChange = length(fwidth(vsample)) * material.dist_falloff;
    let isBorder = smoothstep(material.border - valueChange, material.border + valueChange, voronoi_base.z);
    pbr_input.world_normal = fns::prepare_world_normal(
        apply_hue(in.world_normal, voronoi_base.x),
        double_sided,
        is_front
    );
    pbr_input.N = fns::apply_normal_mapping(
        pbr_input.material.flags,
        mix(grunge_tex_normal.rgb, pbr_input.world_normal.rgb, 0.5),
        double_sided,
        is_front,
        #ifdef VERTEX_TANGENTS
        #ifdef STANDARD_MATERIAL_NORMAL_MAP
        in.world_tangent,
        #endif
        #endif
        in.uv,
        view.mip_bias
    );
    pbr_input.material.base_color = vec4<f32>(apply_hue(material.diffuse_color.rgb, voronoi_base.y * material.color_varience) * pow(1.0 - voronoi_base.x, 2.0) * pow(vec3<f32>(isBorder), vec3(10.0)), 1.0);

    pbr_input.V = fns::calculate_view(in.world_position, pbr_input.is_orthographic);
    return fns::apply_pbr_lighting(pbr_input);
}

fn voronoise(p: vec2<f32>, dep: f32) -> vec3<f32> {
    var md = 10.0;
    var med = 10.0;
    var tcc: vec2<f32>;
    var cc: vec2<f32>;
    let n = floor(p);

    for (var x = -1; x <= 1; x++) {
        for (var y = -1; y <= 1; y++) {
            let g = n + vec2(f32(x), f32(y));
            let o = g + voro_cache[i32(g.x) + (i32(g.y) * 10)].xy;
            let r = o - p;
            let d = length(r);

            if d < md {
                md = d;
                cc = g;
                tcc = r;
            }
        }
    }
    for (var z = -1; z <= 1; z++) {
        for (var w = -1; w <= 1; w++) {
            if dep < 0.002 {
                break;
            }
            let g = n + vec2(f32(z), f32(w));
            let o = g + voro_cache[i32(g.x) + (i32(g.y) * 10)].xy;
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
    let rand = rand22(cc);

    return vec3<f32>(md, rand, med);
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


fn hash2(p: vec2<f32>) -> vec2<f32> {
    // Dave Hoskin's hash as in https://www.shadertoy.com/view/4djSRW
    var p3 = fract(vec3<f32>(p.xyx) * vec3<f32>(.1031, .1030, .0973));
    p3 += dot(p3, p3.yzx + 19.19);
    let o = fract(vec2((p3.x + p3.y) * p3.z, (p3.x + p3.z) * p3.y));
    return o;
}