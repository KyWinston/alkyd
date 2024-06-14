#import bevy_pbr::forward_io::{VertexOutput,FragmentOutput}; 
// #import bevy_pbr::pbr_types::{STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT, PbrInput, pbr_input_new};
// #import bevy_pbr::pbr_functions as fns;
#import bevy_pbr::prepass_utils::{prepass_depth,prepass_normal};
#import bevy_pbr::mesh_view_bindings::view;
// #import bevy_pbr::mesh_functions::{get_model_matrix,mesh_normal_local_to_world,mesh_position_local_to_world, mesh_position_local_to_clip};
// #import bevy_pbr::mesh_bindings::mesh;
#import bevy_pbr::view_transformations::{frag_coord_to_ndc, position_ndc_to_view, position_ndc_to_world};

#import utils::{raymarch_hit,sphere_hit};



struct GalaxyFog {
    diffuse_color: vec4<f32>,
    center: vec3<f32>,
    radius: f32,
}

@group(2) @binding(0) var<uniform> material:GalaxyFog;

@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    return raymarch_hit(view.world_position, material.center, material.radius, material.diffuse_color);
}
