use bevy::asset::{embedded_asset, io::AssetSourceId, AssetPath};
use bevy::{
    prelude::*,
    render::render_resource::{ShaderRef, ShaderType},
};
use std::path::Path;

use crate::compute::{
    traits::{ComputeShader, ComputeWorker},
    worker::AppComputeWorker,
    worker_builder::AppComputeWorkerBuilder,
};

use self::systems::{read_data, run_worker};

pub mod systems;

#[derive(TypePath)]
pub struct VoronoiShader;

impl ComputeShader for VoronoiShader {
    fn shader() -> ShaderRef {
        "embedded://alkyd/utilities/noise.wgsl".into()
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
            .add_staging("texture", &[0.0; 2048 * 2048])
            .add_pass::<VoronoiShader>([2048, 2048, 1], &["texture"])
            .one_shot()
            .build()
    }
}

pub struct UtilitiesPlugin;

impl Plugin for UtilitiesPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "noise.wgsl");
        app.add_systems(Startup, setup);
        app.add_systems(Update, (read_data, run_worker));
    }
}

fn setup() {
    let crate_name = "alkyd";

    let path = Path::new(crate_name).join("noise.wgsl");
    let source = AssetSourceId::from("embedded");
    let asset_path = AssetPath::from_path(&path).with_source(source);

    assert_eq!(asset_path, "embedded://alkyd/noise.wgsl".into());
}
