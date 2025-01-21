use bevy::{math::Vec3, prelude::World, reflect::TypePath};
use bevy_easy_compute::prelude::{
    AppComputeWorker, AppComputeWorkerBuilder, ComputeShader, ComputeWorker, ShaderRef,
};
use rand::{distributions::Uniform, prelude::Distribution};

use crate::{FLUID_SIM_HANDLE, FLUID_SIM_SECOND_PASS_HANDLE};

use super::{resource::FluidParticleBuffer, PARTICLE_COUNT};

#[derive(TypePath)]
struct FluidDensityPass;

#[derive(TypePath)]
struct FluidForcesPass;

impl ComputeShader for FluidDensityPass {
    fn shader() -> ShaderRef {
        FLUID_SIM_HANDLE.into()
    }
}

impl ComputeShader for FluidForcesPass {
    fn shader() -> ShaderRef {
        FLUID_SIM_SECOND_PASS_HANDLE.into()
    }

    fn entry_point<'a>() -> &'a str {
        "calculate_forces"
    }
}

pub struct FluidWorker;

impl ComputeWorker for FluidWorker {
    fn build(world: &mut World) -> AppComputeWorker<Self> {
        let mut particle_container = Vec::<FluidParticleBuffer>::with_capacity(PARTICLE_COUNT);
        let mut rng = rand::thread_rng();
        let unif = Uniform::new_inclusive(-7.3, 7.3);

        for _ in 0..PARTICLE_COUNT {
            let position = Vec3::new(
                unif.sample(&mut rng),
                unif.sample(&mut rng),
                unif.sample(&mut rng),
            );
            particle_container.push(FluidParticleBuffer::new(position));
        }

        let worker = AppComputeWorkerBuilder::new(world)
            .add_staging("particles", &particle_container)
            .add_staging("particles_out", &particle_container)
            .add_pass::<FluidDensityPass>(
                [PARTICLE_COUNT as u32 / 100, 1, 1],
                &["particles", "particles_out"],
            )
            .add_pass::<FluidForcesPass>([PARTICLE_COUNT as u32 / 100, 1, 1], &["particles_out"])
            .add_swap("particles", "particles_out")
            .build();
        worker
    }
}
