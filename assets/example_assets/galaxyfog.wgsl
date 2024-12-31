
#import bevy_pbr::forward_io::VertexOutput;
#import bevy_pbr::prepass_utils::{prepass_depth,prepass_normal};    
#import bevy_pbr::mesh_view_bindings::{globals,view};
#import utils::{raymarch,sdf_cone};
#import noise_gen::FBN;

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
    var ro: vec3<f32> = view.world_position;
    var dist: f32 = 999.0;
    var elapsed_steps = 0;
    let rd: vec3<f32> = normalize(in.world_position - vec4f(ro, 1.0)).xyz;
    var noise_offset: f32;
    for (var x = 0; x < i32(material.steps); x++) {
        if dist <= 30.0 {
            noise_offset = FBN(vec4f(vec3<f32>(ro.x - sin(globals.time), ro.y - globals.time * 1.5, ro.z), globals.time / 8.0));
        } else {
            noise_offset = 0.5;
        }
        let ray: vec4f = raymarch(ro, rd, material.steps, material.prec, sdf_cone(ro - 0.5 + noise_offset, material.radius, 0.1, 2.0));
        ro = ray.xyz;
        dist = ray.a;
        if dist == 0.0 {
            return vec4(vec3f(material.diffuse_color.rgb), 1.0);
        }
        if dist > 50.0 {
             break;
        }
        elapsed_steps++;
    }
    return vec4(vec3f(1.0), 1.0 - dist);
}
