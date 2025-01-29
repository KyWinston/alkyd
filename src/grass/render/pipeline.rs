use crate::{grass::{
    chunk::{GrassChunk, GrassChunkBuffers},
    clump::GrassClumpConfig,
    config::GrassConfigGpu, material::GrassMaterial,
}, utilities::aabb::Aabb2dGpu};
use bevy::{
    pbr::MaterialPipeline,
    prelude::*,
    render::{
        render_resource::{
            binding_types::{
                storage_buffer, storage_buffer_read_only, storage_buffer_read_only_sized,
                storage_buffer_sized, texture_2d, uniform_buffer,
            },
            BindGroupLayout, BindGroupLayoutEntries, CachedComputePipelineId,
            ComputePipelineDescriptor, PipelineCache, ShaderStages, SpecializedComputePipeline,
            SpecializedComputePipelines, TextureSampleType,
        },
        renderer::RenderDevice,
        view::ViewUniform,
    },
};

use super::instance::GrassInstanceData;

#[derive(Resource)]
pub(crate) struct GrassCompactPipeline {
    pub compact_layout: BindGroupLayout,
    pub reset_args_layout: BindGroupLayout,
    pub compact_pipeline_id: CachedComputePipelineId,
    pub reset_args_pipeline_id: CachedComputePipelineId,

    _grass_util_shader: Handle<Shader>,
}

impl FromWorld for GrassCompactPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let compact_layout = render_device.create_bind_group_layout(
            "compact_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    storage_buffer_read_only::<Vec<GrassInstanceData>>(false),
                    storage_buffer_read_only::<Vec<u32>>(false),
                    storage_buffer_read_only::<Vec<u32>>(false),
                    storage_buffer_read_only::<Vec<u32>>(false),
                    storage_buffer::<Vec<GrassInstanceData>>(false),
                    storage_buffer_sized(false, None),
                ),
            ),
        );

        let reset_args_layout = render_device.create_bind_group_layout(
            "reset_args_layout",
            &BindGroupLayoutEntries::single(
                ShaderStages::COMPUTE,
                storage_buffer_sized(false, None),
            ),
        );

        let compact_shader = world
            .resource::<AssetServer>()
            .load("embedded://bevy_procedural_grass/shaders/compact.wgsl");
        let reset_args_shader = world
            .resource::<AssetServer>()
            .load("embedded://bevy_procedural_grass/shaders/reset_args.wgsl");

        let pipeline_cache = world.resource_mut::<PipelineCache>();

        let compact_pipeline_id =
            pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
                label: Some("compute_compact_grass_pipeline".into()),
                layout: vec![compact_layout.clone()],
                push_constant_ranges: Vec::new(),
                zero_initialize_workgroup_memory: false,
                shader: compact_shader.clone(),
                shader_defs: vec![],
                entry_point: "compact".into(),
            });

        let reset_args_pipeline_id =
            pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
                label: Some("reset_args_pipeline".into()),
                zero_initialize_workgroup_memory: false,
                layout: vec![reset_args_layout.clone()],
                push_constant_ranges: Vec::new(),
                shader: reset_args_shader,
                shader_defs: vec![],
                entry_point: "reset_args".into(),
            });

        Self {
            compact_layout,
            reset_args_layout,
            compact_pipeline_id,
            reset_args_pipeline_id,
            _grass_util_shader: world
                .resource::<AssetServer>()
                .load("embedded://bevy_procedural_grass/shaders/grass_util.wgsl"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct GrassGeneratePipelineKey {
    clumps: bool,
}

#[derive(Resource)]
pub struct GrassGeneratePipeline {
    pub chunk_layout: BindGroupLayout,
    pub material_layout: BindGroupLayout,
    pub clump_layout: BindGroupLayout,
    shader: Handle<Shader>,
}
impl FromWorld for GrassGeneratePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let chunk_layout = render_device.create_bind_group_layout(
            "grass_chunk_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    storage_buffer::<Vec<GrassInstanceData>>(false),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    uniform_buffer::<f32>(false),
                    uniform_buffer::<f32>(false),
                    uniform_buffer::<Aabb2dGpu>(false), //TODO: dynamic offset?
                    uniform_buffer::<Aabb2dGpu>(false),
                ),
            ),
        );

        let clump_layout = render_device.create_bind_group_layout(
            "clump_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    uniform_buffer::<Aabb2dGpu>(false),
                    uniform_buffer::<Vec2>(false),
                    storage_buffer_read_only_sized(false, None),
                    storage_buffer_read_only_sized(false, None),
                ),
            ),
        );

        Self {
            chunk_layout,
            material_layout: world
                .resource::<MaterialPipeline<GrassMaterial>>()
                .material_layout
                .clone(),
            clump_layout,
            shader: world
                .resource::<AssetServer>()
                .load("embedded://bevy_procedural_grass/shaders/compute_grass.wgsl"),
        }
    }
}

impl SpecializedComputePipeline for GrassGeneratePipeline {
    type Key = GrassGeneratePipelineKey;

    fn specialize(&self, key: Self::Key) -> ComputePipelineDescriptor {
        let mut layout = vec![self.chunk_layout.clone(), self.material_layout.clone()];
        let mut shader_defs = Vec::new();
        if key.clumps {
            layout.push(self.clump_layout.clone());
            shader_defs.push("CLUMPS".into());
        }

        ComputePipelineDescriptor {
            label: Some("grass_generate_pipeline".into()),
            layout,
            zero_initialize_workgroup_memory: false,
            push_constant_ranges: Vec::new(),
            shader: self.shader.clone(),
            shader_defs,
            entry_point: "main".into(),
        }
    }
}

pub(crate) fn prepare_generate_pipeline(
    mut commands: Commands,
    pipeline_cache: Res<PipelineCache>,
    mut pipelines: ResMut<SpecializedComputePipelines<GrassGeneratePipeline>>,
    generate_pipeline: Res<GrassGeneratePipeline>,
    query: Query<Entity, With<GrassChunk>>,
    clumps: Option<Res<GrassClumpConfig>>,
) {
    for entity in &query {
        let key = GrassGeneratePipelineKey {
            clumps: clumps.is_some(),
        };

        let pipeline_id = pipelines.specialize(&pipeline_cache, &generate_pipeline, key);

        commands
            .entity(entity)
            .insert(GrassGeneratePipelineId(pipeline_id));
    }
}

#[derive(Component)]
pub struct GrassGeneratePipelineId(pub CachedComputePipelineId);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct GrassCullPipelineKey {
    lod: bool,
    shadows: bool,
}

#[derive(Resource)]
pub struct GrassCullPipeline {
    pub cull_layout: BindGroupLayout,
    pub cull_layout_or: BindGroupLayout,
    pub cull_layout_shadow_lod: BindGroupLayout,
    shader: Handle<Shader>,
}
impl FromWorld for GrassCullPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let cull_layout = render_device.create_bind_group_layout(
            "cull_grass_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    storage_buffer_read_only_sized(false, None),
                    storage_buffer::<Vec<u32>>(false),
                    uniform_buffer::<ViewUniform>(true),
                    uniform_buffer::<GrassConfigGpu>(false),
                ),
            ),
        );

        // layout for if shadows OR lod enabled
        let cull_layout_or = render_device.create_bind_group_layout(
            "cull_grass_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    storage_buffer_read_only_sized(false, None),
                    storage_buffer::<Vec<u32>>(false),
                    uniform_buffer::<ViewUniform>(true),
                    uniform_buffer::<GrassConfigGpu>(false),
                    storage_buffer::<Vec<u32>>(false),
                ),
            ),
        );

        let cull_layout_shadow_lod = render_device.create_bind_group_layout(
            "cull_grass_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    storage_buffer_read_only_sized(false, None),
                    storage_buffer::<Vec<u32>>(false),
                    uniform_buffer::<ViewUniform>(true),
                    uniform_buffer::<GrassConfigGpu>(false),
                    storage_buffer::<Vec<u32>>(false),
                    storage_buffer::<Vec<u32>>(false),
                ),
            ),
        );

        Self {
            cull_layout,
            cull_layout_or,
            cull_layout_shadow_lod,
            shader: world
                .resource::<AssetServer>()
                .load("embedded://bevy_procedural_grass/shaders/grass_cull.wgsl"),
        }
    }
}
impl SpecializedComputePipeline for GrassCullPipeline {
    type Key = GrassCullPipelineKey;

    fn specialize(&self, key: Self::Key) -> ComputePipelineDescriptor {
        let layout = match (key.shadows, key.lod) {
            (false, false) => self.cull_layout.clone(),
            (true, false) | (false, true) => self.cull_layout_or.clone(),
            (true, true) => self.cull_layout_shadow_lod.clone(),
        };

        let mut shader_defs = Vec::new();
        if key.shadows {
            shader_defs.push("SHADOW".into());
        }
        if key.lod {
            shader_defs.push("LOD".into());
        }

        ComputePipelineDescriptor {
            label: Some("cull_grass_pipeline".into()),
            layout: vec![layout],
            zero_initialize_workgroup_memory: false,
            push_constant_ranges: Vec::new(),
            shader: self.shader.clone(),
            shader_defs,
            entry_point: "main".into(),
        }
    }
}

pub(crate) fn prepare_cull_pipeline(
    mut commands: Commands,
    pipeline_cache: Res<PipelineCache>,
    mut pipelines: ResMut<SpecializedComputePipelines<GrassCullPipeline>>,
    cull_pipeline: Res<GrassCullPipeline>,
    query: Query<(Entity, &GrassChunkBuffers), With<GrassChunk>>,
) {
    for (entity, buffers) in &query {
        let key = GrassCullPipelineKey {
            lod: buffers.lod_compact_buffers.is_some(),
            shadows: buffers.shadow_compact_buffers.is_some(),
        };

        let pipeline_id = pipelines.specialize(&pipeline_cache, &cull_pipeline, key);

        commands
            .entity(entity)
            .insert(GrassCullPipelineId(pipeline_id));
    }
}

#[derive(Component)]
pub struct GrassCullPipelineId(pub CachedComputePipelineId);
