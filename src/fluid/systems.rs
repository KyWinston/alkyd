use std::f32::consts::PI;

use bevy::{
    color::palettes::tailwind::{BLUE_100, YELLOW_100},
    prelude::*,
};
use rand::{thread_rng, Rng};

use super::{
    components::{FluidParticle, FluidVolume, VolumeDebug, VolumeFilling},
    DAMPING,
};

pub fn init_fluid_particles(
    mut commands: Commands,
    volumes: Query<(Entity, &Transform, &FluidVolume), With<VolumeFilling>>,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<StandardMaterial>>,
) {
    let sphere = mesh.add(Sphere::new(0.05));
    let material = mat.add(Color::srgb_from_array(BLUE_100.to_f32_array_no_alpha()));
    for (ent, transform, vol) in volumes.iter() {
        let mut particle_container = vec![];
        let full_bound = vol.bounds_size;
        let half_bound = full_bound / 2.0;
        let translation = transform.translation;
        for _ in 0..vol.particle_amount {
            let x = (thread_rng().gen_range(0..full_bound.x as i32 * 10000) / 10000) as f32;
            let y = (thread_rng().gen_range(0..full_bound.y as i32 * 10000) / 10000) as f32 - 7.5;
            let z = (thread_rng().gen_range(0..full_bound.z as i32 * 10000) / 10000) as f32;
            let part_ent = commands
                .spawn((
                    FluidParticle::new(ent, 1.0, 3.0),
                    Mesh3d(sphere.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_xyz(
                        (translation.x - half_bound.y) + x,
                        (translation.y - half_bound.y) + y,
                        (translation.z - half_bound.z) + z,
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
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut volumes: Query<(&Transform, &FluidVolume, &mut VolumeDebug)>,
    particles: Query<&FluidParticle, With<Parent>>,
) {
    for mut volume in volumes.iter_mut() {
        gizmos.cuboid(
            Transform::from_scale(volume.1.bounds_size).with_translation(volume.0.translation),
            YELLOW_100,
        );
        volume.2 .0.tick(time.delta());
        if volume.2 .0.just_finished() {
            for particle in particles.iter() {
                if particle.density > 0.0 {
                    println!("density : {:?}", particle.density);
                }
            }
        }
    }
}

pub fn simulate_particles(
    time: Res<Time>,
    mut particles: Query<(&mut Transform, &mut FluidParticle)>,
) {
    for (mut transform, mut particle) in particles.iter_mut() {
        let force = particle.force;
        particle.velocity += force * time.delta_secs() * 0.003;
        transform.translation += particle.velocity * time.delta_secs() * 0.003;
    }
}

pub fn resolve_collisions(
    mut particles: Query<(&mut Transform, &mut FluidParticle), Without<FluidVolume>>,
    mut volumes: Query<(Entity, &FluidVolume)>,
) {
    for (mut part_t, mut particle) in particles.iter_mut() {
        for (ent, volume) in volumes.iter_mut() {
            if ent == particle.parent_volume {
                let half_bound = volume.bounds_size / 2.0 - Vec3::ONE * 0.05;

                if part_t.translation.x.abs() > half_bound.x {
                    part_t.translation.x = half_bound.x * part_t.translation.x.signum();
                    particle.velocity.x *= -1.0 * DAMPING;
                }
                if part_t.translation.y.abs() > half_bound.y {
                    part_t.translation.y = half_bound.y * part_t.translation.y.signum();
                    particle.velocity.y *= -1.0 * DAMPING;
                }
                if part_t.translation.z.abs() > half_bound.z {
                    part_t.translation.z = half_bound.z * part_t.translation.z.signum();
                    particle.velocity.z *= -1.0 * DAMPING;
                }
            }
        }
    }
}

pub fn calcuate_sph(mut particles: Query<(&Transform, &mut FluidParticle)>) {
    let mut density = 0.0;
    let mut combinations = particles.iter_combinations_mut();
    while let Some([(transform_a, mut particle_a), (transform_b, _)]) = combinations.fetch_next() {
        let dist = transform_a.translation - transform_b.translation;
        let radius = particle_a.smoothing_radius;

        let dist_sq = dist.dot(dist);
        if radius * radius * 0.004 > dist_sq * 0.004 {
            density += smooth_kernel_d(radius, dist_sq + 0.004);
        }

        particle_a.density += density + particle_a.mass + 0.00001;
        particle_a.pressure = 2.0 + density - 1.0;
    }
}

pub fn calculate_force(mut particles: Query<(&Transform, &mut FluidParticle)>) {
    let mut pressure = Vec3::ZERO;
    let mut viscosity = Vec3::ZERO;

    let mut combinations = particles.iter_combinations_mut();
    while let Some([(transform_a, mut particle_a), (transform_b, particle_b)]) =
        combinations.fetch_next()
    {
        let dist = transform_a.translation.distance(transform_b.translation);
        if dist < particle_a.smoothing_radius * 2.0 {
            let pressure_dir = (transform_a.translation - transform_b.translation).normalize();
            let mut pressure_contribution = particle_a.mass
                * particle_a.mass
                * spiked_kernel_gradient(particle_a.smoothing_radius, dist, pressure_dir);
            pressure_contribution *= particle_a.pressure
                / (particle_a.density * particle_a.density)
                + particle_b.pressure / (particle_b.density * particle_b.density);

            let mut viscosity_contribution = viscosity
                * (particle_a.mass * particle_a.mass)
                * (particle_b.velocity - particle_a.velocity / particle_b.density);
            viscosity_contribution *= spiked_kernel_d2(particle_a.smoothing_radius, dist);
            pressure += pressure_contribution;
            viscosity += viscosity_contribution;

            particle_a.force = Vec3::new(0.0, 9.81 * particle_a.mass, 0.0) - pressure + viscosity;
        }
    }
}

fn smooth_kernel_d(radius: f32, dist_sq: f32) -> f32 {
    let x = 1.0 - dist_sq / radius * radius;
    315.0 / (64.0 * PI * radius * radius * radius) * x * x * x
}

fn spiked_kernel_d(radius: f32, dist: f32) -> f32 {
    let x = 1.0 - dist / radius;
    -45.0 / (PI * radius * radius * radius * radius) * x * x
}

fn spiked_kernel_d2(radius: f32, dist: f32) -> f32 {
    let x = 1.0 - dist / radius;
    90.0 / (PI * radius * radius * radius * radius * radius) * x
}

fn spiked_kernel_gradient(radius: f32, dist: f32, dir: Vec3) -> Vec3 {
    spiked_kernel_d(radius, dist) * dir
}
