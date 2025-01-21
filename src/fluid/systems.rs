use bevy::{
    color::palettes::tailwind::{BLUE_100, YELLOW_100},
    prelude::*,
};
use bevy_easy_compute::prelude::AppComputeWorker;

use super::{node::FluidWorker, resource::FluidParticleBuffer};

pub fn simulate_fluid_volumes(mut gizmos: Gizmos, worker: ResMut<AppComputeWorker<FluidWorker>>) {
    if !worker.ready() {
        return;
    }

    let fluid_particles = worker.read_vec::<FluidParticleBuffer>("particles_out");
    gizmos.cuboid(Transform::from_scale(Vec3::splat(15.0)), YELLOW_100);

    for particle in fluid_particles.as_slice() {
        gizmos.sphere(
            Isometry3d::from_translation(particle.local_position),
            0.05,
            BLUE_100,
        );
    }
}
