use bevy::prelude::*;
use bevy_app_compute::prelude::{
    AppComputeWorker, AppComputeWorkerBuilder, ComputeShader, ComputeWorker, ShaderRef,
};

#[derive(TypePath)]
struct DistrbutePointsShader;

impl ComputeShader for DistrbutePointsShader {
    fn shader() -> ShaderRef {
        "shader_utils/distribute_points.wgsl".into()
    }
}

#[derive(Resource)]
pub struct DistrbutePointsComputeWorker {
    pub mesh: Handle<Mesh>,
    pub surface: Vec<[Vec3; 3]>,
    pub points: [Vec3; 10000],
}

impl ComputeWorker for DistrbutePointsComputeWorker {
    fn build(world: &mut World) -> AppComputeWorker<Self> {
        let points = world.resource::<DistrbutePointsComputeWorker>().points;
        let surface = world
            .resource::<DistrbutePointsComputeWorker>()
            .surface
            .clone();
        let worker = AppComputeWorkerBuilder::new(world)
            .add_storage("surface", &surface)
            .add_staging("points", &points)
            .add_pass::<DistrbutePointsShader>([64, 100, 100], &["surface","points"])
            .one_shot()
            .build();
        worker
    }
}

pub fn distribute_points(
    mut worker_data: ResMut<DistrbutePointsComputeWorker>,
    mut worker: ResMut<AppComputeWorker<DistrbutePointsComputeWorker>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    let triangles = meshes
        .get(worker_data.mesh.id())
        .unwrap()
        .triangles()
        .expect("invalid mesh");
    for tri in triangles.into_iter() {
        worker_data.surface.push(tri.vertices);
    }
    worker.execute();
}
