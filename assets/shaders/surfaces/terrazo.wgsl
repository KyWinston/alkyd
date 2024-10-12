#import bevy_pbr::pbr_types::{PbrInput};
#import utils::{color_ramp,ColorStop,extend_pbr,voronoi};
#import bevy_pbr::pbr_functions as fns;
#import noise_gen::FBN;
#import bevy_pbr::forward_io::{VertexOutput,FragmentOutput}; 
#import bevy_pbr::pbr_types::{STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT, PbrInput, pbr_input_new};
#import bevy_pbr::mesh_view_bindings::view;

@group(2) @binding(0)
var<uniform> terrazo:Terrazo;

struct Terrazo {
    stops: array<ColorStop,3>,
    scale: f32,
    lc_scale: f32,
    sc_scale: f32,
    lc_size: f32,
    sc_size: f32,
    color_1: vec4f,
    color_2: vec4f,
    roughness: f32,
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool
) -> @location(0) vec4<f32> {

    var pbrInput = extend_pbr(in, is_front);

    let vsample = in.uv.xy * terrazo.scale;
    let fb_noise = vec4f(vec3f(FBN(vsample.xyxy, 8, 1.0, 0.3, 1.0, 2.0)), 1.0);

    let voronoi_l = color_ramp(
        terrazo.stops,
        clamp(voronoi(mix(vsample.xy, fb_noise.xy, vec2f(terrazo.roughness)) * terrazo.lc_scale, 0.0000005, 0u, 1.0).y, 0.0, 1.0)
    );
    let voronoi_s = color_ramp(
        terrazo.stops,
        voronoi(mix(vsample.xy, fb_noise.xy, vec2f(terrazo.roughness)) * terrazo.sc_scale, 0.0000005, 0u, 1.0).y
    );
    pbrInput.material.reflectance = 0.0;
    pbrInput.material.base_color = vec4<f32>(vec3f(mix(vec3f(terrazo.color_2.rgb), vec3f(terrazo.color_1.rgb), select(voronoi_l, voronoi_s, voronoi_l < voronoi_s + 0.3) + 0.6)), 1.0);
    pbrInput.material.perceptual_roughness = 1.0 - pbrInput.material.base_color.x * 0.5;
    pbrInput.V = fns::calculate_view(in.world_position, pbrInput.is_orthographic);
    return fns::apply_pbr_lighting(pbrInput);
}