use std::borrow::Cow;

use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        render_asset::RenderAssets,
        render_graph::{Node, NodeRunError, RenderGraphContext, RenderLabel},
        render_resource::{
            BindGroup, BindGroupEntry, BindingResource, CachedComputePipelineId,
            CachedPipelineState, ComputePassDescriptor, ComputePipelineDescriptor, PipelineCache,
        },
        renderer::{RenderContext, RenderDevice},
        texture::GpuImage,
    },
};

use crate::workers::{
    resources::{NoiseGeneratorPipeline, ShaderHandles},
    systems::NoiseGeneratorState,
    SIZE, WORKGROUP_SIZE,
};

use super::{texture_b::TextureB, texture_c::TextureC, texture_d::TextureD};

#[derive(Resource)]
struct TextureABindGroup {
    texture_a_bind_group: BindGroup,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

#[derive(Clone, Deref, Resource, ExtractResource)]
pub struct TextureA(pub Handle<Image>);

#[derive(Clone, Debug, PartialEq, Eq, Hash, RenderLabel)]
pub struct TextureALabel;

pub fn queue_bind_group_a(
    mut commands: Commands,
    pipeline: Res<NoiseGeneratorPipeline>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    texture_a: Res<TextureA>,
    texture_b: Res<TextureB>,
    texture_c: Res<TextureC>,
    texture_d: Res<TextureD>,
    render_device: Res<RenderDevice>,
    all_shader_handles: Res<ShaderHandles>,
    pipeline_cache: ResMut<PipelineCache>,
) {
    let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: None,
        layout: vec![pipeline.texture_group_layout.clone()],
        push_constant_ranges: vec![],
        shader: all_shader_handles.texture_a_shader.clone(),
        shader_defs: vec!["INIT".to_string().into()],
        entry_point: Cow::from("update"),
    });

    let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: None,
        layout: vec![pipeline.texture_group_layout.clone()],
        push_constant_ranges: vec![],
        shader: all_shader_handles.texture_a_shader.clone(),
        shader_defs: vec![],
        entry_point: Cow::from("update"),
    });

    let view_a = gpu_images.get(&texture_a.0).unwrap();
    let view_b = gpu_images.get(&texture_b.0).unwrap();
    let view_c = gpu_images.get(&texture_c.0).unwrap();
    let view_d = gpu_images.get(&texture_d.0).unwrap();

    let texture_a_bind_group = render_device.create_bind_group(
        Some("texture_a_bind_group"),
        &pipeline.texture_group_layout,
        &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&view_a.texture_view),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::TextureView(&view_b.texture_view),
            },
            BindGroupEntry {
                binding: 2,
                resource: BindingResource::TextureView(&view_c.texture_view),
            },
            BindGroupEntry {
                binding: 3,
                resource: BindingResource::TextureView(&view_d.texture_view),
            },
        ],
    );
    info!("binding a");
    commands.insert_resource(TextureABindGroup {
        texture_a_bind_group,
        init_pipeline,
        update_pipeline,
    });
}

pub struct TextureANode {
    pub state: NoiseGeneratorState,
}

impl Default for TextureANode {
    fn default() -> Self {
        Self {
            state: NoiseGeneratorState::Loading,
        }
    }
}

impl Node for TextureANode {
    fn update(&mut self, world: &mut World) {
        let bind_group = world.resource::<TextureABindGroup>();

        let pipeline_cache = world.resource::<PipelineCache>();

        let init_pipeline_cache = bind_group.init_pipeline;
        let update_pipeline_cache = bind_group.update_pipeline;

        match self.state {
            NoiseGeneratorState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(init_pipeline_cache)
                {
                    self.state = NoiseGeneratorState::Init
                }
            }
            NoiseGeneratorState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(update_pipeline_cache)
                {
                    self.state = NoiseGeneratorState::Update
                }
            }
            NoiseGeneratorState::Update => {}
        }
    }

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        info!("running a");
        let bind_group = world.resource::<TextureABindGroup>();

        let texture_a_bind_group = &bind_group.texture_a_bind_group;

        let init_pipeline_cache = bind_group.init_pipeline;
        let update_pipeline_cache = bind_group.update_pipeline;

        let pipeline_cache = world.resource::<PipelineCache>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, texture_a_bind_group, &[]);

        // select the pipeline based on the current state
        match self.state {
            NoiseGeneratorState::Loading => {}

            NoiseGeneratorState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(init_pipeline_cache)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
            NoiseGeneratorState::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(update_pipeline_cache)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
        }
        Ok(())
    }
}
