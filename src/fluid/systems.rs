use bevy::{
    color::palettes::{
        css::RED,
        tailwind::{BLUE_100, YELLOW_100},
    },
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
    gizmos.sphere(Isometry3d::from_translation(Vec3::ZERO), 0.5, RED);
    for p in fluid_particles.iter() {
        gizmos.sphere(
            Isometry3d::from_translation(Vec3::new(
                p.local_position.x,
                p.local_position.y,
                p.local_position.z,
            )),
            0.1,
            BLUE_100,
        );
    }
}
