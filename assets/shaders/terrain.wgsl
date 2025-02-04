#import bevy_pbr::pbr_types::{PbrInput};
#import utils::{extend_pbr, noise2, color_ramp,ColorStop,voronoi};
#import bevy_pbr::pbr_functions as fns;
#import noise_gen::FBN;
#import bevy_pbr::forward_io::{Vertex, VertexOutput,FragmentOutput}; 
#import bevy_pbr::mesh_view_bindings::view;
#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
};
#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip};

@group(2) @binding(100) var height_map:texture_2d<f32>;
@group(2) @binding(101) var s:sampler;
@group(2) @binding(102) var grow_map:texture_2d<f32>;
@group(2) @binding(103) var s_2:sampler;
@group(2) @binding(104) var normal_map:texture_2d<f32>;
@group(2) @binding(105) var s_3:sampler;




@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let height = textureLoad(height_map, vec2<i32>(vertex.uv * 4096), 0).r;

    out.position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(vertex.position + vec3f(0.0,height * 4.0,0.0), 1.0),
    );

    return out;
}



@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool
) -> @location(0) vec4<f32> {
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    pbr_input.material.perceptual_roughness = 0.8;
    pbr_input.N = textureSample(normal_map,s_3,in.uv).rgb;
    
    pbr_input.V = fns::calculate_view(in.world_position, pbr_input.is_orthographic);
    return fns::apply_pbr_lighting(pbr_input);
}

