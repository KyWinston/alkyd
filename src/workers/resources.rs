use bevy::prelude::*;
use bevy_easy_compute::prelude::{
    AppComputeWorker, AppComputeWorkerBuilder, ComputeShader, ComputeWorker, ShaderRef,
};

use crate::NOISE_COMPUTE_HANDLE;

// use super::WORKGROUP_SIZE;

#[derive(Resource)]
pub struct NoiseComputeWorker;

#[derive(TypePath)]
pub struct NoiseCompute;

impl ComputeShader for NoiseCompute {
    fn shader() -> ShaderRef {
        NOISE_COMPUTE_HANDLE.into()
    }
}

impl ComputeWorker for NoiseComputeWorker {
    fn build(world: &mut World) -> AppComputeWorker<Self> {
        let worker = AppComputeWorkerBuilder::new(world)
            .add_staging("centroids", &[Vec4::ZERO; 1000])
            .add_pass::<NoiseCompute>([100, 100, 1], &[ "centroids"])
            .one_shot()
            .build();
        worker
    }
}

// #[derive(Resource)]
// pub struct SobelComputeWorker;

// #[derive(TypePath)]
// pub struct SobelCompute;

// impl ComputeShader for SobelCompute {
//     fn shader() -> ShaderRef {
//         NOISE_COMPUTE_HANDLE.into()
//     }
// }

// impl ComputeWorker for SobelComputeWorker {
//     fn build(world: &mut World) -> AppComputeWorker<Self> {
//         let worker = AppComputeWorkerBuilder::new(world)
//             .add_staging("rawInput", )
//             .add_pass::<SobelCompute>([10, 10, 1], &[ "centroids"])
//             .build();
//         worker
//     }
// }
