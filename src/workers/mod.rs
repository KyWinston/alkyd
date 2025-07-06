use bevy::prelude::*;
use bevy_app_compute::prelude::AppComputePlugin;

pub struct WorkersPlugin;

pub const WORKGROUP_SIZE: u32 = 64;

impl Plugin for WorkersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AppComputePlugin));
    }
}
