#import bevy_pbr::forward_io::{VertexOutput,FragmentOutput}; 
#import bevy_pbr::pbr_fragment::pbr_input_from_standard_material; 
#import bevy_pbr::pbr_types::{STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT, PbrInput, pbr_input_new},
#import bevy_pbr::pbr_functions as fns;
#import bevy_pbr::view_transformations as vt;
#import bevy_pbr::prepass_utils::prepass_normal;
#import bevy_pbr::mesh_view_bindings::view;


struct Painterly {
    view_normals: u32,
    diffuse_color: vec4<f32>,
    roughness: f32,
    metallic: f32,
    brush_distortion: f32,
    brush_blur: f32,
    brush_angle: f32,
    brush_texture_influence: f32,
    color_varience: f32,
    noise_scale: f32
}

@group(2) @binding(0) var<uniform> material:Painterly;
@group(2) @binding(1) var brush_handle: texture_2d<f32>;
@group(2) @binding(2) var nearest_sampler: sampler;
@group(2) @binding(3) var brush_handle_normal: texture_2d<f32>;
@group(2) @binding(4) var normal_sampler: sampler;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool
) -> @location(0) vec4<f32> {
    var pbr_input: PbrInput = pbr_input_new();
    let grunge_tex = textureSample(brush_handle, nearest_sampler, in.uv * material.noise_scale * 0.5);
    let grunge_tex_normal = textureSample(brush_handle_normal, normal_sampler, in.uv * material.noise_scale);
    let scaling = material.noise_scale * material.brush_distortion;

    pbr_input.material.perceptual_roughness = smooth_knob(material.roughness);
    pbr_input.material.metallic = smooth_knob(material.metallic);
    pbr_input.frag_coord = in.position;
    pbr_input.world_position = in.world_position;
    let double_sided = (pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u;
    pbr_input.world_normal = fns::prepare_world_normal(
        in.world_normal,
        double_sided,
        is_front
    );
    let voronoi_base = voronoise(mix(pbr_input.world_normal.rg * material.noise_scale, mix(vec3(snoise2(in.uv * scaling)), grunge_tex.rgb, smooth_knob(material.brush_texture_influence)).xy, 0.5), smooth_knob(material.brush_angle), smooth_knob(material.brush_blur));

    pbr_input.N = fns::apply_normal_mapping(
        pbr_input.material.flags,
        apply_hue(apply_hue(pbr_input.world_normal, length(grunge_tex) * smooth_knob(material.brush_texture_influence)), voronoi_base),
        double_sided,
        is_front,
        #ifdef VERTEX_TANGENTS
        #ifdef STANDRD_MATERIAL_NORMAL_MAP
        in.world_tangent,
        #endif
        #endif
        in.uv,
        view.mip_bias,
    );
    pbr_input.material.base_color = vec4(apply_hue(material.diffuse_color.rgb, voronoi_base * material.color_varience), 1.0);
    pbr_input.V = fns::calculate_view(in.world_position, pbr_input.is_orthographic);
    if material.view_normals == 1 {
        return vec4(pbr_input.N, 1.0);
    } else {
        return fns::apply_pbr_lighting(pbr_input);
    }
}

fn hash_three(p: vec2<f32>) -> vec3<f32> {
    let q: vec3<f32>= vec3(dot(p, vec2(127.1, 311.7)),
        dot(p, vec2(269.5, 183.3)),
        dot(p, vec2(419.2, 371.9)));
    return fract(sin(q) * 43758.5453);
}


fn smooth_knob(in:f32) -> f32{
    return clamp(in / 50.0, 0.0, 1.0);
}

fn voronoise(p: vec2<f32>, dist: f32, blur: f32) -> f32{
    let k = 1. + 63. * pow(1. - blur, 4.);

    let i = floor(p);
    let f = fract(p);
    var a = vec2(0.0);
    // for (var z: i32 = -2; z <= 2; z++) {
        for (var y: i32 = -2; y <= 2; y++) {
            for (var x: i32 = -2; x <= 2; x++) {
                let g = vec2f(f32(x), f32(y));
                let o = hash_three(i + g) * vec3f(dist, dist, 1.);
                let d = g - f + o.xy;
                let w = pow(1. - smoothstep(0., 1.414, length(d)), k);
                a += vec2(o.z * w,w);
            }
        }
    // }
    return a.x / a.y;
}

fn mod289(x: vec2<f32>) -> vec2<f32> {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

fn mod289_3(x: vec3<f32>) -> vec3<f32> {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

fn permute3(x: vec3f) -> vec3f {
    return mod289_3(((x * 34.) + 1.) * x);
}

fn apply_hue(col: vec3<f32>, hueAdjust: f32) -> vec3<f32> {
    let k = vec3(0.57735, 0.57735, 0.57735);
    let cosAngle = cos(hueAdjust);
    return col * cosAngle + cross(k, col) * sin(hueAdjust) + k * dot(k, col) * (1.0 - cosAngle);
}



//  MIT License. © Ian McEwan, Stefan Gustavson, Munrocket
fn snoise2(v: vec2f) -> f32 {
    let C = vec4(
        0.211324865405187, // (3.0-sqrt(3.0))/6.0
        0.366025403784439, // 0.5*(sqrt(3.0)-1.0)
        -0.577350269189626, // -1.0 + 2.0 * C.x
        0.024390243902439 // 1.0 / 41.0
    );

    // First corner
    var i = floor(v + dot(v, C.yy));
    let x0 = v - i + dot(i, C.xx);

    // Other corners
    var i1 = select(vec2(0., 1.), vec2(1., 0.), x0.x > x0.y);

    // x0 = x0 - 0.0 + 0.0 * C.xx ;
    // x1 = x0 - i1 + 1.0 * C.xx ;
    // x2 = x0 - 1.0 + 2.0 * C.xx ;
    var x12 = x0.xyxy + C.xxzz;
    x12.x = x12.x - i1.x;
    x12.y = x12.y - i1.y;

    // Permutations
    i = mod289(i); // Avoid truncation effects in permutation

    var p = permute3(permute3(i.y + vec3(0., i1.y, 1.)) + i.x + vec3(0., i1.x, 1.));
    var m = max(0.5 - vec3(dot(x0, x0), dot(x12.xy, x12.xy), dot(x12.zw, x12.zw)), vec3(0.));
    m *= m;
    m *= m;

    // Gradients: 41 points uniformly over a line, mapped onto a diamond.
    // The ring size 17*17 = 289 is close to a multiple of 41 (41*7 = 287)
    let x = 2. * fract(p * C.www) - 1.;
    let h = abs(x) - 0.5;
    let ox = floor(x + 0.5);
    let a0 = x - ox;

    // Normalize gradients implicitly by scaling m
    // Approximation of: m *= inversesqrt( a0*a0 + h*h );
    m *= 1.79284291400159 - 0.85373472095314 * (a0 * a0 + h * h);

    // Compute final noise value at P
    let g = vec3(a0.x * x0.x + h.x * x0.y, a0.yz * x12.xz + h.yz * x12.yw);
    return 130. * dot(m, g);
}