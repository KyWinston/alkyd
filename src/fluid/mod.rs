use bevy::{asset::load_internal_asset, prelude::*};
use bevy_easy_compute::prelude::{AppComputePlugin, AppComputeWorkerPlugin};
use node::FluidWorker;
use systems::simulate_fluid_volumes;

use crate::FLUID_SIM_HANDLE;

pub mod components;
pub mod node;

pub mod resource;
pub mod systems;

pub struct FluidPlugin;

impl Plugin for FluidPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            FLUID_SIM_HANDLE,
            "../../assets/fluids/sph_fluid.wgsl",
            Shader::from_wgsl
        );
        app.add_plugins(AppComputePlugin)
            .add_plugins(AppComputeWorkerPlugin::<FluidWorker>::default())
            .add_systems(Update, simulate_fluid_volumes);
    }
}
