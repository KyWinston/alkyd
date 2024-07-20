use std::borrow::Cow;

use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        render_resource::{
            binding_types::texture_storage_2d, BindGroup, BindGroupLayout, BindGroupLayoutEntries,
            CachedComputePipelineId, ComputePipelineDescriptor, PipelineCache, ShaderStages,
            StorageTextureAccess, TextureFormat,
        },
        renderer::RenderDevice,
    },
    utils::HashMap,
};

use super::SHADER_ASSET_PATH;

#[derive(Resource, Clone, ExtractResource)]
pub struct NoiseImages(pub HashMap<String, [Handle<Image>;2]>);

#[derive(Resource)]
pub struct NoiseGeneratorBindGroups(pub [BindGroup; 2]);

#[derive(Resource)]
pub struct NoiseGeneratorPipeline {
    pub texture_bind_group_layout: BindGroupLayout,
    pub init_pipeline: CachedComputePipelineId,
    pub update_pipeline: CachedComputePipelineId,
}

impl FromWorld for NoiseGeneratorPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let texture_bind_group_layout = render_device.create_bind_group_layout(
            "noise",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage_2d(TextureFormat::R32Float, StorageTextureAccess::ReadOnly),
                    texture_storage_2d(TextureFormat::R32Float, StorageTextureAccess::WriteOnly),
                ),
            ),
        );

        let shader = world.load_asset(SHADER_ASSET_PATH);
        let pipeline_cache = world.resource::<PipelineCache>();
        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
        });
        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
        });
        NoiseGeneratorPipeline {
            texture_bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}
