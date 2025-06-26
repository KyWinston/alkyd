use bevy::prelude::*;
use bevy_app_compute::prelude::{
    AppComputeWorker, AppComputeWorkerBuilder, ComputeShader, ComputeWorker, ShaderRef,
};

use crate::UV_PAINT_HANDLE;

#[derive(TypePath)]
struct UvPaintShader;

impl ComputeShader for UvPaintShader {
    fn shader() -> ShaderRef {
        UV_PAINT_HANDLE.into()
    }
}

#[derive(Resource)]
pub struct BrushCoord(pub Option<Vec<f32>>);

#[derive(Resource)]
pub struct UvPaintComputeWorker;

impl ComputeWorker for UvPaintComputeWorker {
    fn build(world: &mut World) -> AppComputeWorker<Self> {
        let worker = AppComputeWorkerBuilder::new(world)
            // Add a uniform variable
            .add_uniform("brush_point", &[0., 0.])
            // Add a staging buffer, it will be available from
            // both CPU and GPU land.
            // Create a compute pass from your compute shader
            // and define used variables
            .add_pass::<UvPaintShader>([4, 1, 1], &["brush_point"])
            .one_shot()
            .build();

        worker
    }
}

pub fn on_click_compute(
    brush: Res<BrushCoord>,
    mut compute_worker: ResMut<AppComputeWorker<UvPaintComputeWorker>>,
) {
    if brush.0.is_some() {
        compute_worker.execute();
    }
}

pub fn read_data(mut compute_worker: ResMut<AppComputeWorker<UvPaintComputeWorker>>) {
    if !compute_worker.ready() {
        return;
    };

    let result: Vec<f32> = compute_worker.read_vec("values");

    compute_worker.write_slice("values", &result);

    println!("got {:?}", result)
}
