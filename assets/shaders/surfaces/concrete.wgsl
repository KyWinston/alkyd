#import bevy_pbr::pbr_types::{PbrInput};
#import utils::{extend_pbr, noise2, color_ramp,ColorStop,voronoi};
#import bevy_pbr::pbr_functions as fns;
#import noise_gen::FBN;
#import bevy_pbr::forward_io::{VertexOutput,FragmentOutput}; 
#import bevy_pbr::mesh_view_bindings::view;

@group(2) @binding(0)
var<uniform> concrete:Concrete;

struct Concrete {
    stops: array<ColorStop,3>,
    scale: f32,
    roughness: f32,
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool
) -> @location(0) vec4<f32> {

    var pbrInput = extend_pbr(in, is_front);

    let voronoi_base = voronoi(sin(in.uv * concrete.scale) * concrete.scale, 0.0000005, 0u, 1.0);
    let noise = noise2(in.uv * concrete.scale);
    pbrInput.material.base_color = vec4f(color_ramp(concrete.stops, mix(vec3f(voronoi_base.x), vec3f(noise), 0.8).x), 1.0);
    pbrInput.material.perceptual_roughness = pbrInput.material.base_color.x * 0.5;

    pbrInput.V = fns::calculate_view(in.world_position, pbrInput.is_orthographic);
    return fns::apply_pbr_lighting(pbrInput);
}