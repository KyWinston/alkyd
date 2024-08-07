use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        render_asset::RenderAssets,
        render_graph::{self, RenderLabel},
        render_resource::*,
        renderer::{RenderContext, RenderDevice},
        texture::GpuImage,
    },
};

use std::borrow::Cow;

use crate::workers::resources::ShaderHandles;
use crate::workers::{resources::NoiseGeneratorPipeline, WORKGROUP_SIZE};
use crate::workers::{systems::NoiseGeneratorState, SIZE};

use super::{texture_a::TextureA, texture_b::TextureB, texture_d::TextureD};

#[derive(Resource)]
struct TextureCBindGroup {
    texture_c_bind_group: BindGroup,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

#[derive(Clone, Deref, Resource, ExtractResource)]
pub struct TextureC(pub Handle<Image>);

#[derive(Clone, Debug, PartialEq, Eq, Hash, RenderLabel)]
pub struct TextureCLabel;

pub fn queue_bind_group_c(
    mut commands: Commands,
    pipeline: Res<NoiseGeneratorPipeline>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    texture_a_image: Res<TextureA>,
    texture_b_image: Res<TextureB>,
    texture_c_image: Res<TextureC>,
    texture_d_image: Res<TextureD>,
    render_device: Res<RenderDevice>,
    all_shader_handles: Res<ShaderHandles>,
    pipeline_cache: ResMut<PipelineCache>,
) {
    let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: None,
        layout: vec![pipeline.texture_group_layout.clone()],
        push_constant_ranges: vec![],
        shader: all_shader_handles.texture_c_shader.clone(),
        shader_defs: vec!["INIT".to_string().into()],
        entry_point: Cow::from("init"),
    });

    let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: None,
        layout: vec![pipeline.texture_group_layout.clone()],
        push_constant_ranges: vec![],
        shader: all_shader_handles.texture_c_shader.clone(),
        shader_defs: vec![],
        entry_point: Cow::from("update"),
    });

    let texture_a_view = gpu_images.get(&texture_a_image.0).unwrap();
    let texture_b_view = gpu_images.get(&texture_b_image.0).unwrap();
    let texture_c_view = gpu_images.get(&texture_c_image.0).unwrap();
    let texture_d_view = gpu_images.get(&texture_d_image.0).unwrap();

    let texture_c_bind_group = render_device.create_bind_group(
        Some("bind_group_c"),
        &pipeline.texture_group_layout,
        &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&texture_a_view.texture_view),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::TextureView(&texture_b_view.texture_view),
            },
            BindGroupEntry {
                binding: 2,
                resource: BindingResource::TextureView(&texture_c_view.texture_view),
            },
            BindGroupEntry {
                binding: 3,
                resource: BindingResource::TextureView(&texture_d_view.texture_view),
            },
        ],
    );
    info!("binding c");
    commands.insert_resource(TextureCBindGroup {
        texture_c_bind_group,
        init_pipeline,
        update_pipeline,
    });
}

pub struct TextureCNode {
    pub state: NoiseGeneratorState,
}

impl Default for TextureCNode {
    fn default() -> Self {
        Self {
            state: NoiseGeneratorState::Loading,
        }
    }
}

impl render_graph::Node for TextureCNode {
    fn update(&mut self, world: &mut World) {
        let bind_group = world.resource::<TextureCBindGroup>();

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
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        info!("running c");

        let bind_group = world.resource::<TextureCBindGroup>();

        let texture_c_bind_group = &bind_group.texture_c_bind_group;

        let init_pipeline_cache = bind_group.init_pipeline;
        let update_pipeline_cache = bind_group.update_pipeline;

        let pipeline_cache = world.resource::<PipelineCache>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, texture_c_bind_group, &[]);

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
