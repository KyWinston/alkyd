use bevy::{
    prelude::*,
    render::{
        render_resource::{
            BindGroup, CachedComputePipelineId, ComputePassDescriptor, PipelineCache,
        },
        renderer::RenderContext,
    },
};

use super::{ComputeState, SIZE, WORKGROUP_SIZE};
#[allow(clippy::too_many_arguments)]
pub fn init_compute(
    context: &mut RenderContext,
    world: &World,
    bind_group: &[BindGroup],
    cache: CachedComputePipelineId,
    bind_idx: u32,
    offsets: &[u32],
    state: ComputeState,
    desc: ComputePassDescriptor,
) {
    let pipeline_cache = world.resource::<PipelineCache>();

    let mut pass = context.command_encoder().begin_compute_pass(&desc);

    match state {
        ComputeState::Loading => {}
        ComputeState::Init => {
            let init_pipeline = pipeline_cache.get_compute_pipeline(cache).unwrap();
            pass.set_bind_group(bind_idx, &bind_group[0], offsets);
            pass.set_pipeline(init_pipeline);
            pass.dispatch_workgroups(
                SIZE / WORKGROUP_SIZE,
                SIZE / WORKGROUP_SIZE,
                SIZE / WORKGROUP_SIZE,
            );
        }
        ComputeState::Finished => {}
        ComputeState::Update => {}
    }
}
