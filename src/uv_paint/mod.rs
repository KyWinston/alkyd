use bevy::prelude::*;
use bevy_app_compute::prelude::{AppComputePlugin, AppComputeWorkerPlugin};
use systems::{on_click_compute, read_data, UvPaintComputeWorker};

pub mod systems;

pub struct  UvPaintPlugin;

impl Plugin for UvPaintPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(AppComputePlugin)
            .add_plugins(AppComputeWorkerPlugin::<UvPaintComputeWorker>::default())
            .add_systems(Update, (on_click_compute, read_data))
            .run();
    }
}
