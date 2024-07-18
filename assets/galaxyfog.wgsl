
#import bevy_pbr::forward_io::VertexOutput;
#import bevy_pbr::prepass_utils::{prepass_depth,prepass_normal};
#import bevy_pbr::mesh_view_bindings::{globals,view};
#import utils::{raymarch_hit};

struct GalaxyFog {
    diffuse_color: vec4<f32>,
    center: vec3<f32>,
    radius: f32,
    steps: u32,
    prec: f32
}

@group(2) @binding(0) var<uniform> material:GalaxyFog;

@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {

    return raymarch_hit(in.world_position, material.center, material.radius, material.diffuse_color, material.steps, material.prec);
}
