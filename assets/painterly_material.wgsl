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
    // brush_distortion: f32,
    // brush_blur: f32,
    // brush_angle: f32,
    // brush_texture_influence: f32,
    color_varience: f32,
    tiling_period: vec3<f32>,
    noise_scale: f32
}

@group(2) @binding(0) var<uniform> material:Painterly;
// @group(2) @binding(1) var brush_handle: texture_2d<f32>;
// @group(2) @binding(2) var nearest_sampler: sampler;
// @group(2) @binding(3) var brush_handle_normal: texture_2d<f32>;
// @group(2) @binding(4) var normal_sampler: sampler;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool
) -> @location(0) vec4<f32> {
    var pbr_input: PbrInput = pbr_input_new();
    // let scaling = material.noise_scale * material.brush_distortion;
    // let grunge_tex = textureSample(brush_handle, nearest_sampler, in.uv * material.noise_scale * 0.2);
    // let grunge_tex_normal = textureSample(brush_handle_normal, normal_sampler, in.uv * material.noise_scale);
    let voronoi_base = voronoise_3t(vec3(in.uv,0.5) * material.noise_scale, material.tiling_period).y;
    pbr_input.material.perceptual_roughness = smooth_knob(material.roughness);
    pbr_input.material.metallic = smooth_knob(material.metallic);
    pbr_input.frag_coord = in.position;
    pbr_input.world_position = in.world_position;
    let double_sided = (pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u;
    pbr_input.world_normal = fns::prepare_world_normal(
        apply_hue(in.world_normal, voronoi_base),
        double_sided,
        is_front
    );

    pbr_input.N = fns::apply_normal_mapping(
        pbr_input.material.flags,
        pbr_input.world_normal,
        double_sided,
        is_front,
        #ifdef VERTEX_TANGENTS
        #ifdef STANDARD_MATERIAL_NORMAL_MAP
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
    // return vec4(vec3(voronoi_base), 1.0);
    } else {
        return fns::apply_pbr_lighting(pbr_input);
    }
}

fn srandom(pos: vec3<f32>) -> f32 {
    return -1. + 2. * fract(sin(dot(pos.xyz, vec3(70.9898, 78.233, 32.4355))) * 43758.5453123);
}

fn srandom3(p: vec3<f32>) -> vec3<f32> {
    let x = vec3(dot(p, vec3(127.1, 311.7, 74.7)),
        dot(p, vec3(269.5, 183.3, 246.1)),
        dot(p, vec3(113.5, 271.9, 124.6)));
    return -1. + 2. * fract(sin(x) * 43758.5453123);
}


fn smooth_knob(in: f32) -> f32 {
    return clamp(in / 50.0, 0.0, 1.0);
}

// fn voronoise(p: vec2<f32>, dist: f32, blur: f32) -> f32{
//     let k = 1. + 63. * pow(1. - blur, 4.);

//     let i = floor(p);
//     let f = fract(p);
//     var a = vec2(0.0);
//     // for (var z: i32 = -2; z <= 2; z++) {
//         for (var y: i32 = -2; y <= 2; y++) {
//             for (var x: i32 = -2; x <= 2; x++) {
//                 let g = vec2f(f32(x), f32(y));
//                 let o = hash_three(i + g) * vec3f(dist, dist, 1.);
//                 let d = g - f + o.xy;
//                 let w = pow(1. - smoothstep(0., 1.414, length(d)), k);
//                 a += vec2(o.z * w,w);
//             }
//         }
//     // }
//     return a.x / a.y;
// }

fn voronoise_3t(x: vec3<f32>, period: vec3<f32>) -> vec3<f32> {
    var base = floor(x);

    var min_dist = 10.0;
    var to_closest: vec3<f32>;
    var closest: vec3<f32>;
    for (var x1 = -1; x1 <= 1; x1++) {
        for (var y1 = -1; y1 <= 1; y1++) {
            for (var z1 = -1; z1 <= 1; z1++) {
                let bc = base + vec3(f32(x1), f32(y1), f32(z1));
                var tc = mod_3(bc, period);
                let rc = bc + srandom3(tc);
                let to_cell = rc - x;
                let cell_dist = length(to_cell);
                if cell_dist < min_dist {
                    min_dist = cell_dist;
                    closest = bc;
                    to_closest = to_cell;
                }
            }
        }
    }

    var min_edge_dist = 10.0;
    for (var x2 = -1; x2 <= 1; x2++) {
        for (var y2 = -1; y2 <= 1; y2++) {
            for (var z2 = -1; z2 <= 1; z2++) {
                let bc = base + vec3(f32(x2), f32(y2), f32(z2));
                let tc = mod_3(bc, vec3<f32>(period));
                let rc = bc + srandom3(tc);
                let to_cell = rc - x;

                let cell_diff = abs(closest - bc);

                if cell_diff.x + cell_diff.y + cell_diff.z >= 0.1 {
                    let to_center = (to_closest + to_cell) * 0.5;
                    let diff = normalize(to_cell - to_closest);
                    let edge_dist = dot(to_center, diff);
                    min_edge_dist = min(min_edge_dist, edge_dist);
                }
            }
        }
    }
    var rand = srandom(closest);
    return vec3<f32>(min_dist, rand, min_edge_dist);
}

// fn mod289(x: vec2<f32>) -> vec2<f32> {
//     return x - floor(x * (1.0 / 289.0)) * 289.0;
// }

fn mod_3(x: vec3<f32>, divisor: vec3<f32>) -> vec3<f32> {
    let pos_x = x % divisor + divisor;
    return pos_x % divisor;
}

// fn permute3(x: vec3f) -> vec3f {
//     return mod289_3(((x * 34.) + 1.) * x);
// }

fn apply_hue(col: vec3<f32>, hueAdjust: f32) -> vec3<f32> {
    let k = vec3(0.57735, 0.57735, 0.57735);
    let cosAngle = cos(hueAdjust);
    return col * cosAngle + cross(k, col) * sin(hueAdjust) + k * dot(k, col) * (1.0 - cosAngle);
}



// //  MIT License. © Ian McEwan, Stefan Gustavson, Munrocket
// fn snoise2(v: vec2f) -> f32 {
//     let C = vec4(
//         0.211324865405187, // (3.0-sqrt(3.0))/6.0
//         0.366025403784439, // 0.5*(sqrt(3.0)-1.0)
//         -0.577350269189626, // -1.0 + 2.0 * C.x
//         0.024390243902439 // 1.0 / 41.0
//     );

//     // First corner
//     var i = floor(v + dot(v, C.yy));
//     let x0 = v - i + dot(i, C.xx);

//     // Other corners
//     var i1 = select(vec2(0., 1.), vec2(1., 0.), x0.x > x0.y);

//     // x0 = x0 - 0.0 + 0.0 * C.xx ;
//     // x1 = x0 - i1 + 1.0 * C.xx ;
//     // x2 = x0 - 1.0 + 2.0 * C.xx ;
//     var x12 = x0.xyxy + C.xxzz;
//     x12.x = x12.x - i1.x;
//     x12.y = x12.y - i1.y;

//     // Permutations
//     i = mod289(i); // Avoid truncation effects in permutation

//     var p = permute3(permute3(i.y + vec3(0., i1.y, 1.)) + i.x + vec3(0., i1.x, 1.));
//     var m = max(0.5 - vec3(dot(x0, x0), dot(x12.xy, x12.xy), dot(x12.zw, x12.zw)), vec3(0.));
//     m *= m;
//     m *= m;

//     // Gradients: 41 points uniformly over a line, mapped onto a diamond.
//     // The ring size 17*17 = 289 is close to a multiple of 41 (41*7 = 287)
//     let x = 2. * fract(p * C.www) - 1.;
//     let h = abs(x) - 0.5;
//     let ox = floor(x + 0.5);
//     let a0 = x - ox;

//     // Normalize gradients implicitly by scaling m
//     // Approximation of: m *= inversesqrt( a0*a0 + h*h );
//     m *= 1.79284291400159 - 0.85373472095314 * (a0 * a0 + h * h);

//     // Compute final noise value at P
//     let g = vec3(a0.x * x0.x + h.x * x0.y, a0.yz * x12.xz + h.yz * x12.yw);
//     return 130. * dot(m, g);
// }