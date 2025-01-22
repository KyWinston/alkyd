use bevy::{asset::load_internal_asset, prelude::*};
use bevy_easy_compute::prelude::{AppComputePlugin, AppComputeWorkerPlugin};
use node::FluidWorker;
use systems::simulate_fluid_volumes;

use crate::{FLUID_CONSTS, FLUID_SIM_HANDLE, FLUID_SIM_SECOND_PASS_HANDLE};

pub mod components;
pub mod node;

pub mod resource;
pub mod systems;

pub const PARTICLE_COUNT: usize = 1000;
pub struct FluidPlugin;

impl Plugin for FluidPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            FLUID_SIM_SECOND_PASS_HANDLE,
            "../../assets/fluids/sph_second.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            FLUID_SIM_HANDLE,
            "../../assets/fluids/sph_fluid.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            FLUID_CONSTS,
            "../../assets/fluids/consts.wgsl",
            Shader::from_wgsl
        );

        
        app.add_plugins(AppComputePlugin)
            .add_plugins(AppComputeWorkerPlugin::<FluidWorker>::default())
            .add_systems(Update, simulate_fluid_volumes);
    }
}
