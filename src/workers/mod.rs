use bevy::prelude::*;
use resources::NoiseComputeWorker;
use bevy_easy_compute::prelude::{AppComputeWorker, AppComputeWorkerPlugin};

pub mod resources;
pub mod systems;
pub struct WorkersPlugin;

pub const DISPLAY_FACTOR: u32 = 4;
pub const SIZE: (u32, u32) = (1920 / DISPLAY_FACTOR, 1080 / DISPLAY_FACTOR);
pub const WORKGROUP_SIZE: u32 = 8;

impl Plugin for WorkersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AppComputeWorkerPlugin::<NoiseComputeWorker>::default())
            .add_systems(
                PostStartup,
                |mut compute_worker: ResMut<AppComputeWorker<NoiseComputeWorker>>| {
                    compute_worker.execute();
                },
            );
    }
}
