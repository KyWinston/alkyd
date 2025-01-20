use bevy::prelude::*;
use systems::{
    calcuate_sph, calculate_force, debug_fluid_volumes, init_fluid_particles, simulate_particles,
};

pub mod components;
pub mod systems;

pub struct FluidPlugin;

pub const DAMPING: f32 = 0.4;
pub const GAS_CONSTANT: f32 = 2.0;
pub const REST_DENSITY: f32 = 10.0;

pub const RADIUS: f32 = 2.0;
pub const RADIUS2: f32 = 4.0;
pub const RADIUS3: f32 = 8.0;
pub const RADIUS4: f32 = 16.0;
pub const RADIUS5: f32 = 32.0;

pub const STEP: f32 = 0.004;

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
                )
                    .chain(),
            ),
        );
    }
}
