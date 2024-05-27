use crate::compute::{
    traits::{ComputeShader, ComputeWorker},
    worker::AppComputeWorker,
    worker_builder::AppComputeWorkerBuilder,
};
use bevy::{
    prelude::*,
    render::render_resource::{ShaderRef, ShaderType},
};

use self::systems::{read_data, run_worker};

pub mod systems;

#[derive(TypePath)]
pub struct VoronoiShader;

impl ComputeShader for VoronoiShader {
    fn shader() -> ShaderRef {
        "noise.wgsl".into()
    }
}

#[derive(ShaderType)]
struct Properties {
    distort: f32,
    influence: f32,
    angle: f32,
    blur: f32,
}

#[derive(Resource)]
pub struct VoronoiWorker;

impl ComputeWorker for VoronoiWorker {
    fn build(world: &mut World) -> AppComputeWorker<Self> {
        AppComputeWorkerBuilder::new(world)
            .add_uniform("cell_size", &5.)
            .add_staging("centroids", &[Vec4::ZERO; 100])
            .add_pass::<VoronoiShader>([10, 10, 1], &["cell_size", "centroids"])
            .one_shot()
            .build()
    }
}

pub struct UtilitiesPlugin;

impl Plugin for UtilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (read_data, run_worker));
    }
}
