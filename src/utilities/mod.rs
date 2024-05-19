use bevy::{
    ecs::{system::Resource, world::World},
    reflect::TypePath,
    render::render_resource::ShaderType,
};
use bevy_app_compute::prelude::{
    AppComputeWorker, AppComputeWorkerBuilder, ComputeShader, ComputeWorker,
};

pub mod systems;

#[derive(TypePath)]
pub struct VoronoiShader;

impl ComputeShader for VoronoiShader {
    fn shader() -> bevy_app_compute::prelude::ShaderRef {
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
