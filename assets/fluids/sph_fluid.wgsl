#define_import_path sph_fluids

#import global_values::PI;

@group(0) @binding(0) var<storage> fluid_particles: array<FluidParticle>;
@group(0) @binding(1) var<storage, read_write> fluid_particles_out: array<FluidParticle>;


struct FluidParticle {
    local_position: vec3<f32>,
    velocity: vec3<f32>,
    mass: f32,
    pressure: f32,
    density: f32,
    force: vec3<f32>
}

const DAMPING: f32 = 0.5;
const GAS_CONSTANT: f32 = 2.0;
const REST_DENSITY: f32 = 1.0;

const RADIUS: f32 = 2.0;
const RADIUS2: f32 = 4.0;
const RADIUS3: f32 = 8.0;
const RADIUS4: f32 = 16.0;
const RADIUS5: f32 = 32.0;

const STEP:f32 = 0.04;

@compute @workgroup_size(100)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {

    let total = arrayLength(&fluid_particles);
    let index = invocation_id.x;

    if index >= total {
        return;
    }

    var density = 0.0;
    var target_particle = fluid_particles[index];

    for (var i = 0; i < i32(total); i++) {
        if u32(i) == index {
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

