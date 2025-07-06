#define_import_path foliage;
#import bevy_pbr::mesh_functions::view_transformations::{position_view_to_world, mesh_position_world_to_clip};
#import bevy_pbr::pbr_functions::prepare_world_normal;
#import bevy_pbr::mesh_functions::{get_world_from_local,mesh_normal_local_to_world,mesh_position_local_to_world, mesh_position_local_to_clip};
#import bevy_pbr::mesh_view_bindings::view;

struct FoliageProperties{
    density:f32,
    count:u32,
    variance:f32,
    grass_color:vec4<f32>
}

@group(2) @binding(0)
var<uniform> foliage_props:FoliageProperties;

@group(2) @binding(1)
var<storage,read_write> points:array<vec3<f32>>;

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
    @location(5) color:vec4<f32>
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.index = vertex.instance_index;
    let vertex_position = points[out.index] + vertex.position * rotate_y(foliage_props.variance) - vec3(0.0,0.2,0.0);
    out.normal = normalize(mesh_normal_local_to_world(vertex.normal,out.index));
    out.world_position = mesh_position_local_to_world(get_world_from_local(out.index),vec4(vertex_position,1.0)).xyz;
    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4(vertex_position,1.0),
    );
    out.uv = vertex.uv;  
    out.color = foliage_props.grass_color;
    return out;
}



fn rotate_y(angle:f32)->mat3x3<f32>{
    return mat3x3(
        vec3(cos(angle),0.0,sin(angle)),
        vec3(0.0,1.0,0.0),
        vec3(-sin(angle),0.0,cos(angle))
    );
}