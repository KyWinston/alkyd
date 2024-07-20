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

use crate::workers::{
    resources::NoiseGeneratorPipeline, systems::NoiseGeneratorState, SHADER_ASSET_PATH, SIZE,
    WORKGROUP_SIZE,
};

use super::{texture_a::TextureA, texture_c::TextureC, texture_d::TextureD};

#[derive(Resource)]
struct TextureBBindGroup {
    texture_b_bind_group: BindGroup,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

#[derive(Clone, Deref, ExtractResource, Resource)]
pub struct TextureB(pub Handle<Image>);

#[derive(Clone, Debug, PartialEq, Eq, Hash, RenderLabel)]
pub struct TextureBLabel;

pub fn queue_bind_group_b(
    mut commands: Commands,
    pipeline: Res<NoiseGeneratorPipeline>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    texture_a_image: Res<TextureA>,
    texture_b_image: Res<TextureB>,
    texture_c_image: Res<TextureC>,
    texture_d_image: Res<TextureD>,
    render_device: Res<RenderDevice>,
    asset_server: Res<AssetServer>,
    pipeline_cache: ResMut<PipelineCache>,
) {
    let shader = asset_server.load(SHADER_ASSET_PATH);
    let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: None,
        layout: vec![pipeline.texture_bind_group_layout.clone()],
        push_constant_ranges: vec![],
        shader: shader.clone(),
        shader_defs: vec!["INIT".to_string().into()],
        entry_point: Cow::from("update"),
    });

    let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: None,
        layout: vec![pipeline.texture_bind_group_layout.clone()],
        push_constant_ranges: vec![],
        shader: shader,
        shader_defs: vec![],
        entry_point: Cow::from("update"),
    });

    let texture_a_view = gpu_images.get(&texture_a_image.0).unwrap();
    let texture_b_view = gpu_images.get(&texture_b_image.0).unwrap();
    let texture_c_view = gpu_images.get(&texture_c_image.0).unwrap();
    let texture_d_view = gpu_images.get(&texture_d_image.0).unwrap();

    let texture_b_bind_group = render_device.create_bind_group(
        Some("binding b"),
        &pipeline.texture_bind_group_layout,
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

    commands.insert_resource(TextureBBindGroup {
        texture_b_bind_group,
        init_pipeline,
        update_pipeline,
    });
}

pub struct TextureBNode {
    pub state: NoiseGeneratorState,
}

impl Default for TextureBNode {
    fn default() -> Self {
        Self {
            state: NoiseGeneratorState::Loading,
        }
    }
}

impl render_graph::Node for TextureBNode {
    fn update(&mut self, world: &mut World) {
        let bind_group = world.resource::<TextureBBindGroup>();

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
        let bind_group = world.resource::<TextureBBindGroup>();
        let texture_b_bind_group = &bind_group.texture_b_bind_group;

        let init_pipeline_cache = bind_group.init_pipeline;
        let update_pipeline_cache = bind_group.update_pipeline;

        let pipeline_cache = world.resource::<PipelineCache>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        // pass.set_bind_group(0, texture_a_bind_group, &[]);
        pass.set_bind_group(0, texture_b_bind_group, &[]);

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
