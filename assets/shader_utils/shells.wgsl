#define_import_path shells;

#import bevy_pbr::mesh_functions::view_transformations::{mesh_position_world_to_clip};
#import bevy_pbr::pbr_functions::prepare_world_normal;
#import bevy_pbr::mesh_functions::{get_world_from_local,mesh_normal_local_to_world,mesh_position_local_to_world, mesh_position_local_to_clip};
 

struct ShellProperties{
    length:f32,
    density:f32,
    thickness:f32,
    count:u32,
    variance:vec2<f32>,
    shell_color:vec4<f32>
}

@group(2) @binding(0)
var<uniform> shell_props:ShellProperties;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv:vec2<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) index: u32,
    @location(1) position:vec4<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) world_position: vec3<f32>,
    @location(4) uv:vec2<f32>,
    @location(5) density:f32,
    @location(6) count:u32,
    @location(7) variance:vec2<f32>,
    @location(8) color:vec4<f32>
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.index = vertex.instance_index;
    out.position = vec4(vertex.position,1.0);

    var shell_height = f32(out.index) / f32(shell_props.count);

    var vertex_position = out.position.xyz;

    vertex_position += vertex.normal.xyz * shell_props.length * shell_height;
    out.normal = normalize(mesh_normal_local_to_world(vertex.normal,out.index));
    out.world_position = mesh_position_local_to_world(get_world_from_local(out.index),vec4<f32>(vertex_position,1.0)).xyz;
    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(vertex_position, 1.0),
    );
    out.uv = vertex.uv;
    out.density = shell_props.density;
    out.count = shell_props.count;
    out.variance = shell_props.variance;
    out.color = shell_props.shell_color;

    return out;
}



