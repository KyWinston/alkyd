use std::f32::consts::PI;

use bevy::{
    color::palettes::tailwind::{BLUE_100, YELLOW_100},
    prelude::*,
};
use rand::{thread_rng, Rng};

use super::{
    components::{FluidParticle, FluidVolume, VolumeDebug, VolumeFilling},
    DAMPING, GAS_CONSTANT, RADIUS, RADIUS2, RADIUS3, RADIUS4, RADIUS5, REST_DENSITY, STEP,
};

pub fn init_fluid_particles(
    mut commands: Commands,
    volumes: Query<(Entity, &Transform, &FluidVolume), With<VolumeFilling>>,
) {
    for (ent, transform, vol) in volumes.iter() {
        let mut particle_container = vec![];
        let full_bound = vol.bounds_size;
        let half_bound = full_bound / 2.0;
        let translation = transform.translation;
        for _ in 0..vol.particle_amount {
            let x = (thread_rng().gen_range(0..full_bound.x as i32 * 10000) / 10000) as f32;
            let y = (thread_rng().gen_range(0..full_bound.y as i32 * 10000) / 10000) as f32;
            let z = (thread_rng().gen_range(0..full_bound.z as i32 * 10000) / 10000) as f32;
            let part_ent = commands
                .spawn((
                    FluidParticle::new(ent, 1.0),
                    Transform::from_xyz(
                        translation.x - half_bound.x + x,
                        translation.y - half_bound.y + y,
                        translation.z - half_bound.z + z,
                    ),
                ))
                .id();
            particle_container.push(part_ent);
        }
        commands.entity(ent).add_children(&particle_container);
        commands.entity(ent).remove::<VolumeFilling>();
    }
}

pub fn debug_fluid_volumes(
    mut gizmos: Gizmos,
    mut volumes: Query<(Entity, &Transform, &FluidVolume, &mut VolumeDebug)>,
) {
    for volume in volumes.iter_mut() {
        gizmos.cuboid(
            Transform::from_scale(volume.2.bounds_size).with_translation(volume.1.translation),
            YELLOW_100,
        );
    }
}

pub fn simulate_particles(
    mut gizmos: Gizmos,
    mut particles: Query<(&mut Transform, &mut FluidParticle), Without<FluidVolume>>,
    volumes: Query<(Entity, &Transform, &FluidVolume)>,
) {
    for (ent, _, volume) in volumes.iter() {
        for (mut transform, mut particle) in particles.iter_mut() {
            if particle.parent_volume == ent {
                let force = particle.force;
                let mass = particle.mass;

                let mut vel = (particle.velocity + force / mass) * STEP;

                transform.translation += vel * STEP;

                let half_bound = volume.bounds_size / 2.0 - Vec3::ONE * RADIUS;

                //min bounds
                if transform.translation.x.abs() > half_bound.x {
                    transform.translation.x = half_bound.x * transform.translation.x.signum();
                    vel.x *= -1.0 * DAMPING;
                }
                if transform.translation.y.abs() > half_bound.y {
                    transform.translation.y = half_bound.y * transform.translation.y.signum();
                    vel.y *= -1.0 * DAMPING;
                }
                if transform.translation.z.abs() > half_bound.z {
                    transform.translation.z = half_bound.z * transform.translation.z.signum();
                    vel.z *= -1.0 * DAMPING;
                }

                particle.velocity = vel;

                gizmos.sphere(
                    Isometry3d::from_translation(transform.translation),
                    0.05,
                    BLUE_100,
                );
            }
        }
    }
}

pub fn calcuate_sph(mut particles: Query<(&Transform, &mut FluidParticle)>) {
    let mut density = 0.0;
    let mut combinations = particles.iter_combinations_mut();
    while let Some([(transform_a, mut particle_a), (transform_b, _)]) = combinations.fetch_next() {
        let dist = transform_a.translation - transform_b.translation;

        let dist_sq = dist.dot(dist);
        if RADIUS2 * 0.004 > dist_sq * 0.004 {
            density += smooth_kernel_d(dist_sq * 0.004);
        }

        particle_a.density = density * particle_a.mass + 0.000001;
        particle_a.pressure = GAS_CONSTANT * (particle_a.density - REST_DENSITY);
    }
}

pub fn calculate_force(mut particles: Query<(&Transform, &mut FluidParticle)>) {
    let mut pressure = Vec3::ZERO;
    let mut viscosity = Vec3::ZERO;
    let mut combinations = particles.iter_combinations_mut();
    while let Some([(transform_a, mut particle_a), (transform_b, particle_b)]) =
        combinations.fetch_next()
    {
        let mass_sq = particle_a.mass * particle_a.mass;
        let density_sq = particle_a.density * particle_a.density;
        let density_sq_b = particle_b.density * particle_b.density;

        let dist = transform_b.translation.distance(transform_a.translation);
        if dist < RADIUS * 2.0 {
            let pressure_dir = (transform_a.translation - transform_b.translation).normalize();
            let mut pressure_contribution = mass_sq * spiked_kernel_gradient(dist, pressure_dir);
            pressure_contribution *=
                particle_a.pressure / density_sq + particle_b.pressure / density_sq_b;
            let mut viscosity_contribution =
                viscosity * mass_sq * (particle_b.velocity - particle_a.velocity)
                    / particle_b.density;
            viscosity_contribution *= spiked_kernel_d2(dist);
            pressure += pressure_contribution;
            viscosity += viscosity_contribution;
        }
        particle_a.force = Vec3::new(0.0, -9.81 * particle_a.mass, 0.0) - pressure + viscosity;
    }
}

fn smooth_kernel_d(dist_sq: f32) -> f32 {
    let x = 1.0 - dist_sq / RADIUS2;
    315.0 / (64.0 * PI * RADIUS3) * x * x * x
}

fn spiked_kernel_d(dist: f32) -> f32 {
    let x = 1.0 - dist / RADIUS;
    -45.0 / (PI * RADIUS4) * x * x
}

fn spiked_kernel_d2(dist: f32) -> f32 {
    let x = 1.0 - dist / RADIUS;
    90.0 / (PI * RADIUS5) * x
}

fn spiked_kernel_gradient(dist: f32, dir: Vec3) -> Vec3 {
    spiked_kernel_d(dist) * dir
}
