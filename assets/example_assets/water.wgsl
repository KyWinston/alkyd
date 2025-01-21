#import bevy_pbr::forward_io::{VertexOutput};
#import sph_fluids::{ FluidVolume, FluidParticle };

#import bevy_pbr::forward_io::{VertexOutput};
#import bevy_pbr::pbr_functions as fns;
#import bevy_pbr::pbr_types::{PbrInput,pbr_input_new};
#import bevy_pbr::mesh_view_bindings::{globals,view};
#import utils::{raymarch,conemarch,sdf_cone,map};
#import bevy_pbr::utils::coords_to_viewport_uv;


@group(0) @binding(0) var<storage, read_write> fluid_volumes: array<FluidVolume>;
@group(0) @binding(1) var<storage, read_write> fluid_particles: array<FluidParticle>;

@group(2) @binding(0) var<uniform> fluid: FluidMaterial;

struct FluidMaterial {
    diffuse_color: vec4f,
}

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    var ro: vec3<f32> = view.world_position;
    var dist: f32 = 999.0;
    let tolerance = 300.0;
    let rd: vec3<f32> = normalize(in.world_position - vec4f(ro, 1.0)).xyz;
    var noise_offset: f32;

    for (var x = 0; x < i32(material.steps); x++) {
        let nearest = 999.0;

        if dist > 20.0 {
            noise_offset = 0.0;
        } else {
            noise_offset = (textureSample(first_half, s, vec2<f32>((ro.x + cos(globals.time)), ro.y - globals.time * 3.5)) * textureSample(second_half, s_2, vec2<f32>((ro.z + sin(globals.time)), 1.0))).r;
        }
        let ray: vec4f = raymarch(ro, rd, sdf_cone(ro + noise_offset, material.radius, 0.1, 2.0));
        ro = ray.xyz;
        dist = ray.a;
        if dist <= 1.0 / tolerance {
            for (var x = 0; x < i32(material.steps / 2); x++) {
                let transmit_ray = raymarch(ro, rd, sdf_cone(ro + noise_offset, material.radius / 2.0, 0.01, 1.0));

                if transmit_ray.a <= 1.0 / tolerance {
                    return vec4(vec3f(1.0, 1.0, 0.0), 1.0);
                }
                ro += rd * transmit_ray.a;
            }
            return vec4(vec3f(material.diffuse_color.rgb), 1.0);
        }
    }
    return vec4(vec3f(0.0), 1.0 - dist);
    return fluid.diffuse_color;
}

