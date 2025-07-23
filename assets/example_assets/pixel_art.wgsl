#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import global_values::bayer_slices
#import bevy_pbr::mesh_view_bindings::view
#import bevy_render::maths::PI
#import bevy_pbr::pbr_types::{PbrInput, pbr_input_new},
#import bevy_pbr::{
    prepass_utils::{prepass_depth,prepass_normal},
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions as fns
}
#endif
const bayer = mat4x4(0, 8, 2, 10,
    12, 4, 14, 6,
    3, 11, 1, 9,
    15, 7, 13, 5) / 16.0;

struct PixelArtMaterial {
    color: vec4<f32>,
    outline: vec4<f32>,
    bayer_count: u32,
    bayer_scale: f32,
    quantize_steps: u32,
    metallic: f32,
    specular: f32
}

@group(2) @binding(0)
var<uniform> material: PixelArtMaterial;

@fragment
fn fragment(
    #ifdef MULTISAMPLED
    @builtin(sample_index) sample_index: u32,
    #endif
    mesh: VertexOutput,
) -> FragmentOutput {
    #ifdef PREPASS_PIPELINE
    let out = deferred_output(in, pbr_input);
    #else
        #ifndef MULTISAMPLED
    let sample_index = 0u;
        #endif

    var pbr_input = pbr_input_new();

    var out: FragmentOutput;
    pbr_input.material.perceptual_roughness = material.specular;
    pbr_input.material.metallic = material.metallic;

    pbr_input.material.base_color = material.color;

    let depth = prepass_depth(mesh.position, sample_index);
    pbr_input.N = prepass_normal(mesh.position, sample_index);

    var depth_dist: f32;
    var normal_dist: vec3<f32>;
    var normal_sum: f32;
    var uvs: array<vec4<f32>,4>;
    uvs[0] = vec4(-1.0, 1.0, vec2(0.0));
    uvs[1] = vec4(1.0, 1.0, vec2(0.0));
    uvs[2] = vec4(-1.0, -1.0, vec2(0.0));
    uvs[3] = vec4(1.0, -1.0, vec2(0.0));

    for (var i = 0; i < 4; i++) {
        var d = prepass_depth(mesh.position + uvs[i], sample_index);
        depth_dist += depth - d;

        var n = prepass_normal(mesh.position + uvs[i], sample_index).rgb;
        normal_dist = pbr_input.N.rgb - n;
        let normal_diff = dot(normal_dist, vec3(1.0));
        let normal_ind = smoothstep(-0.01, 0.01, normal_diff);
        normal_sum += dot(normal_dist, normal_dist) * normal_ind;
    }
    let ind = sqrt(normal_sum);
    let norm_edge = step(0.01, ind);

    let depth_edge = step(0.01, depth_dist);
    if depth_edge > 0.0 {
        pbr_input.material.base_color = mix(pbr_input.material.base_color, pbr_input.material.base_color * 0.5, depth_edge);
    } else {
        pbr_input.material.base_color = mix(pbr_input.material.base_color, pbr_input.material.base_color + 0.3, norm_edge);
    }

    out.color = fns::apply_pbr_lighting(pbr_input);

    #endif

    return out;
}

