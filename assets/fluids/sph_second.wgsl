#define_import_path sph_fluids

#import global_values::PI;
#import fluid_consts::{STEP, DAMPING, RADIUS, RADIUS3, RADIUS4, RADIUS5};

@group(0) @binding(0) var<storage, read_write> fluid_particles_out: array<FluidParticle>;

struct FluidParticle {
    local_position: vec3<f32>,
    velocity: vec3<f32>,
    mass: f32,
    pressure: f32,
    density: f32,
    force: vec3<f32>
}


@compute @workgroup_size(100)
fn calculate_forces(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let total = arrayLength(&fluid_particles_out);
    let index = invocation_id.x;

    if index >= total {
        return;
    }

    var target_particle = fluid_particles_out[index];

    let mass_sq = target_particle.mass * target_particle.mass;
    let density_sq = target_particle.density * target_particle.density;

    var pressure = vec3<f32>(0.0);
    var viscosity = vec3<f32>(0.0);

    for (var i = 0; i < i32(total); i++) {
        if u32(i) == index {
            continue;
        }
        let dist = distance(fluid_particles_out[i].local_position, target_particle.local_position);

        if dist < RADIUS * 2.0 {
            let pressure_dir = normalize(target_particle.local_position - fluid_particles_out[i].local_position);

            var pressure_contribution = mass_sq * spiked_kernel_gradient(dist, pressure_dir);
            pressure_contribution *= (target_particle.pressure / density_sq + fluid_particles_out[i].pressure / (fluid_particles_out[i].density * fluid_particles_out[i].density));

            var viscosity_contribution = viscosity * mass_sq * (fluid_particles_out[i].velocity - target_particle.velocity) / fluid_particles_out[i].density;
            viscosity_contribution *= spiked_kernel_d2(dist);

            pressure += pressure_contribution;
            viscosity += viscosity_contribution;
        }
    }

    target_particle.force = vec3<f32>(0.0, -9.81 * target_particle.mass, 0.0) - pressure + viscosity;
    fluid_particles_out[index] = target_particle;
}

@compute @workgroup_size(64)
fn integrate(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let total = arrayLength(&fluid_particles_out);
    let index = invocation_id.x;

    if index >= total {
        return;
    }
    var target_particle = fluid_particles_out[index];


    var vel = target_particle.velocity + (target_particle.force / target_particle.mass) * STEP;

    target_particle.local_position += vel * STEP;

    let half_bound = 15.0 / 2.0 - vec3(1.0) * 0.1;

    if abs(target_particle.local_position.x) > half_bound.x {
        vel.x *= -1.0 * DAMPING;

        target_particle.local_position.x = half_bound.x * sign(target_particle.local_position.x);
    }

    if abs(target_particle.local_position.y) > half_bound.y {
        vel.y *= -1.0 * DAMPING;
        target_particle.local_position.y = half_bound.y * sign(target_particle.local_position.y);
    }

    if abs(target_particle.local_position.z) > half_bound.z {
        vel.z *= -1.0 * DAMPING;
        target_particle.local_position.z = half_bound.z * sign(target_particle.local_position.z);
    }

    target_particle.velocity = vel;
    fluid_particles_out[index] = target_particle;
}

fn spiked_kernel_d(dist: f32) -> f32 {
    let x = 1.0 - dist / RADIUS;
    return -45.0 / (PI * RADIUS4) * x * x;
}

fn spiked_kernel_d2(dist: f32) -> f32 {
    let x = 1.0 - dist / RADIUS;
    return 90.0 / (PI * RADIUS5) * x;
}

fn spiked_kernel_gradient(dist: f32, dir: vec3f) -> vec3f {
    return spiked_kernel_d(dist) * dir;
}