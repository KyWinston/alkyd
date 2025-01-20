use bevy::prelude::*;
use systems::{
    calcuate_sph, calculate_force, debug_fluid_volumes, init_fluid_particles, resolve_collisions,
    simulate_particles,
};

pub mod components;
pub mod systems;

pub struct FluidPlugin;

pub const DAMPING: f32 = 0.4;
pub const TARGET_DENSITY: f32 = 0.1;
pub const RAD_8: f32 = 6561.0;

impl Plugin for FluidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                init_fluid_particles,
                (
                    calcuate_sph,
                    calculate_force,
                    simulate_particles,
                    debug_fluid_volumes,
                    resolve_collisions,
                )
                    .chain(),
            ),
        );
    }
}
