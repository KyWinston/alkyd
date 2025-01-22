#define_import_path sph_fluids

#import global_values::PI;
#import fluid_consts::{RADIUS2, RADIUS3, GAS_CONSTANT, REST_DENSITY}

@group(0) @binding(0) var<storage, read> fluid_particles: array<FluidParticle>;
@group(0) @binding(1) var<storage, read_write> fluid_particles_out: array<FluidParticle>;

struct FluidParticle {
    local_position: vec3<f32>,
    velocity: vec3<f32>,
    mass: f32,
    pressure: f32,
    density: f32,
    force: vec3<f32>
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {

    let total = arrayLength(&fluid_particles);
    let index = invocation_id.x;

    if index >= total {
        return;
    }

    var target_particle = fluid_particles[index];
    var density = 0.0;
    for (var i = 0; i < i32(total); i++) {
        if u32(i) == index{
            continue;
        }

        let dist = target_particle.local_position - fluid_particles[i].local_position;

        let dist_sq = dot(dist, dist);
        if RADIUS2 * 0.004 > dist_sq * 0.004 {
            density += smooth_kernel_d(dist_sq * 0.004);
        }
    }

    target_particle.density = density * target_particle.mass + 0.0000001;
    target_particle.pressure = GAS_CONSTANT * (target_particle.density - REST_DENSITY);

    fluid_particles_out[index] = target_particle;
}

fn smooth_kernel_d(dist_sq: f32) -> f32 {
    let x = 1.0 - dist_sq / RADIUS2;
    return 315.0 / (64.0 * PI * RADIUS3) * x * x * x;
}