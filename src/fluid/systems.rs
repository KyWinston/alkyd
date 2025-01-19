use std::f32::consts::PI;

use bevy::{
    color::palettes::tailwind::{BLUE_100, YELLOW_100},
    prelude::*,
};
use rand::Rng;

use super::{
    components::{FluidParticle, FluidVolume, VolumeDebug, VolumeFilling},
    DAMPING, TARGET_DENSITY,
};

pub fn init_fluid_particles(
    mut commands: Commands,
    volumes: Query<(Entity, &Transform, &FluidVolume), With<VolumeFilling>>,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<StandardMaterial>>,
) {
    let sphere = mesh.add(Sphere::new(0.1));
    let material = mat.add(Color::srgb_from_array(BLUE_100.to_f32_array_no_alpha()));
    for (ent, transform, vol) in volumes.iter() {
        let mut particle_container = vec![];
        for _ in 0..vol.particle_amount {
            let x =
                (rand::thread_rng().gen_range(0..vol.bounds_size.x as i32 * 1000) / 1000) as f32;
            let y =
                (rand::thread_rng().gen_range(0..vol.bounds_size.y as i32 * 1000) / 1000) as f32;
            let z =
                (rand::thread_rng().gen_range(0..vol.bounds_size.z as i32 * 1000) / 1000) as f32;

            let part_ent = commands
                .spawn((
                    FluidParticle::new(ent, 1.0, 7.0),
                    Mesh3d(sphere.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_xyz(
                        (transform.translation.x - vol.bounds_size.x / 2.0) + x,
                        (transform.translation.y - vol.bounds_size.y / 2.0) + y,
                        (transform.translation.z - vol.bounds_size.z / 2.0) + z,
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
    volumes: Query<(&Transform, &FluidVolume), With<VolumeDebug>>,
) {
    for volume in volumes.iter() {
        gizmos.cuboid(
            Transform::from_scale(volume.1.bounds_size).with_translation(volume.0.translation),
            YELLOW_100,
        );
        gizmos.sphere(
            Isometry3d::from_translation(volume.0.translation),
            0.1,
            BLUE_100,
        );
    }
}

pub fn simulate_particles(
    time: Res<Time>,
    mut particles: Query<(&mut Transform, &mut FluidParticle), With<Parent>>,
) {
    for (mut transform, mut particle) in particles.iter_mut() {
        particle.velocity += Vec3::NEG_Y * 9.8 * time.delta_secs();
        if particle.density > 0.0 {
            let pressure_acc = particle.pressure / particle.density;
            particle.velocity += pressure_acc * time.delta_secs();
        }

        transform.translation += particle.velocity * time.delta_secs();
        particle.density = 0.0;
        particle.pressure = Vec3::ZERO;
    }
}

pub fn resolve_collisions(
    mut particles: Query<(&mut Transform, &mut FluidParticle), Without<FluidVolume>>,
    mut volumes: Query<(Entity, &FluidVolume)>,
) {
    for (mut part_t, mut particle) in particles.iter_mut() {
        for (ent, volume) in volumes.iter_mut() {
            if ent == particle.parent_volume {
                let half_bound = volume.bounds_size / 2.0 - Vec3::ONE * 0.01;

                if part_t.translation.x.abs() > half_bound.x {
                    part_t.translation.x = half_bound.x * part_t.translation.x.signum();
                    particle.velocity.x *= 1.0 * DAMPING;
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
    let mut combinations = particles.iter_combinations_mut();
    while let Some([(transform_a, mut particle_a), (transform_b, _)]) = combinations.fetch_next() {
        let dist = transform_a.translation.distance(transform_b.translation);
        let influence = smooth_kernel(particle_a.smoothing_radius, dist);
        particle_a.density += particle_a.mass * influence;
        if particle_a.density > 0.0 {
            particle_a.pressure = calculate_pressure(
                transform_a.translation,
                transform_b.translation,
                particle_a.smoothing_radius,
                particle_a.density,
                particle_a.mass,
            );
        }
        println!("{:?}", particle_a.density);
    }
}

fn calculate_pressure(point_a: Vec3, point_b: Vec3, radius: f32, density: f32, mass: f32) -> Vec3 {
    let dist = point_a.distance(point_b);
    if dist == 0.0 {
        return Vec3::ZERO;
    }
    let dir = (point_a - point_b) / dist;
    let slope = smooth_kernel(radius, dist);
    -(density - TARGET_DENSITY) * dir * slope * mass / density
}

fn smooth_kernel(radius: f32, dist: f32) -> f32 {
    if dist >= radius {
        return 0.0;
    }
    let volume = PI * radius.powf(8.0) / 4.0;
    let value = (radius - dist).clamp(0.0, radius - dist);
    if volume == 0.0 {
        return 0.0;
    }
    value * value * value / volume
}
