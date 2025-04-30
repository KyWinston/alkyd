use std::borrow::Cow;

use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        render_resource::{
            binding_types::{texture_storage_2d, uniform_buffer},
            BindGroup, BindGroupLayout, BindGroupLayoutEntries, CachedComputePipelineId,
            ComputePipelineDescriptor, PipelineCache, ShaderStages, ShaderType,
            StorageTextureAccess, TextureFormat,
        },
        renderer::RenderDevice,
    },
};
use bytemuck::{Pod, Zeroable};

use crate::TEX_GEN_HANDLE;

#[derive(Resource, Clone, ExtractResource)]
pub struct TexGenImage {
    pub texture_0: Handle<Image>,
    pub texture_1: Handle<Image>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, ShaderType, Pod, Zeroable)]
pub struct NoiseProperties {
    pub octaves: i32,
    pub lacunarity: f32,
    pub gain: f32,
    pub amplitude: f32,
    pub frequency: f32,
}

impl Default for NoiseProperties {
    fn default() -> Self {
        Self {
            octaves: 4,
            lacunarity: 2.0,
            gain: 0.03,
            amplitude: 1.0,
            frequency: 1.0,
        }
    }
}

#[derive(Resource)]
pub struct TexGenImageBindGroup(pub [BindGroup; 1]);

#[derive(Resource)]
pub struct TexGenPipeline {
    pub texture_bind_group_layout: BindGroupLayout,
    pub init_pipeline: CachedComputePipelineId,
}

impl FromWorld for TexGenPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let texture_bind_group_layout = render_device.create_bind_group_layout(
            None,
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage_2d(TextureFormat::R32Float, StorageTextureAccess::ReadWrite),
                    texture_storage_2d(TextureFormat::R32Float, StorageTextureAccess::ReadWrite),
                    uniform_buffer::<NoiseProperties>(false),
                ),
            ),
        );

        let shader = TEX_GEN_HANDLE;
        let pipeline_cache = world.resource::<PipelineCache>();
        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
            zero_initialize_workgroup_memory: false,
        });

        TexGenPipeline {
            texture_bind_group_layout,
            init_pipeline,
        }
    }
}
