use bevy::{
    ecs::query::QueryItem,
    math::bounding::Aabb2d,
    prelude::*,
    render::{
        extract_component::ExtractComponent,
        render_resource::{
            Buffer, BufferDescriptor, BufferInitDescriptor, BufferUsages, DrawIndexedIndirectArgs,
        },
        renderer::RenderDevice,
    }, utils::HashMap,
};

use super::{lod::GrassLODMesh, render::instance::GrassInstanceData, GrassHeightMap};
use crate::{prefix_sum::PrefixSumBuffers, utilities::aabb::Aabb2dGpu};

#[derive(Component, Clone)]
pub struct GrassChunk {
    pub grass_entity: Entity,
    pub aabb: Aabb2d,
    pub instance_count: usize,
    pub scan_workgroup_count: u32,
}

#[derive(Component, Clone)]
pub struct GrassChunkBuffers {
    pub aabb_buffer: Buffer,
    pub instance_buffer: Buffer,
    pub compact_buffers: GrassChunkCullBuffers,
    pub lod_compact_buffers: Option<GrassChunkCullBuffers>,
    pub(crate) shadow_compact_buffers: Option<GrassChunkCullBuffers>,
}

#[derive(Clone)]
pub struct GrassChunkCullBuffers {
    pub vote_buffer: Buffer,
    pub compact_buffer: Buffer,
    pub indirect_args_buffer: Buffer,
    pub(crate) prefix_sum_buffers: PrefixSumBuffers,
}
impl GrassChunkCullBuffers {
    fn create_buffers(
        render_device: &RenderDevice,
        instance_count: usize,
        scan_workgroup_count: u32,
        index_count: u32,
    ) -> Self {
        Self {
            vote_buffer: render_device.create_buffer(&BufferDescriptor {
                label: Some("vote_buffer"),
                size: (std::mem::size_of::<u32>() * instance_count) as u64,
                usage: BufferUsages::STORAGE,
                mapped_at_creation: false,
            }),
            compact_buffer: render_device.create_buffer(&BufferDescriptor {
                label: Some("compact_buffer"),
                size: (std::mem::size_of::<GrassInstanceData>() * instance_count) as u64,
                usage: BufferUsages::VERTEX | BufferUsages::STORAGE,
                mapped_at_creation: false,
            }),
            indirect_args_buffer: render_device.create_buffer_with_data(&BufferInitDescriptor {
                label: Some("indirect_indexed_args"),
                contents: DrawIndexedIndirectArgs {
                    index_count,
                    instance_count: 0,
                    first_index: 0,
                    base_vertex: 0,
                    first_instance: 0,
                }
                .as_bytes(),
                usage: BufferUsages::STORAGE | BufferUsages::INDIRECT,
            }),
            prefix_sum_buffers: PrefixSumBuffers::create_buffers(
                &render_device,
                instance_count as u32,
                scan_workgroup_count,
            ),
        }
    }
}

impl GrassChunkBuffers {
    pub(crate) fn create_buffers(
        render_device: &RenderDevice,
        aabb: Aabb2d,
        instance_count: usize,
        index_count: u32,
        scan_workgroup_count: u32,
        lod_index_count: Option<u32>,
        grass_shadows: bool,
    ) -> Self {
        let compact_buffers = GrassChunkCullBuffers::create_buffers(
            render_device,
            instance_count,
            scan_workgroup_count,
            index_count,
        );
        let lod_compact_buffers = if let Some(lod_index_count) = lod_index_count {
            Some(GrassChunkCullBuffers::create_buffers(
                render_device,
                instance_count,
                scan_workgroup_count,
                lod_index_count,
            ))
        } else {
            None
        };
        let shadow_compact_buffers = if grass_shadows {
            Some(GrassChunkCullBuffers::create_buffers(
                render_device,
                instance_count,
                scan_workgroup_count,
                index_count,
            ))
        } else {
            None
        };

        Self {
            aabb_buffer: render_device.create_buffer_with_data(&BufferInitDescriptor {
                label: Some("aabb_buffer"),
                contents: bytemuck::cast_slice(&[Aabb2dGpu::from(aabb)]),
                usage: BufferUsages::UNIFORM,
            }),
            instance_buffer: render_device.create_buffer(&BufferDescriptor {
                label: Some("instance_buffer"),
                size: (std::mem::size_of::<GrassInstanceData>() * instance_count) as u64,
                usage: BufferUsages::VERTEX | BufferUsages::STORAGE,
                mapped_at_creation: false,
            }),
            compact_buffers,
            lod_compact_buffers,
            shadow_compact_buffers,
        }
    }
}

impl ExtractComponent for GrassChunk {
    type QueryData = (
        &'static GrassChunk,
        &'static GrassChunkBuffers,
        &'static GrassLODMesh,
    );
    type QueryFilter = ();
    type Out = (GrassChunk, GrassChunkBuffers, GrassLODMesh);

    fn extract_component(item: QueryItem<'_, Self::QueryData>) -> Option<Self::Out> {
        Some((item.0.clone(), item.1.clone(), item.2.clone()))
    }
}

#[derive(Reflect, Component, Clone)]
pub struct Grass {
    pub chunk_count: UVec2,
    pub density: f32,
    pub height_map: Option<GrassHeightMap>,
    pub y_offset: f32,
}

impl Default for Grass {
    fn default() -> Self {
        Self {
            chunk_count: UVec2::splat(1),
            density: 10.0,
            height_map: None,
            y_offset: 0.0,
        }
    }
}

// TODO: rename this i dont like it
#[derive(Component, Clone)]
pub struct GrassGpuInfo {
    pub aabb: Aabb2d,
    pub chunk_size: Vec2,
    pub aabb_buffer: Buffer,
    pub height_scale_buffer: Buffer,
    pub height_offset_buffer: Buffer,

    pub instance_count: usize,
    pub workgroup_count: u32,
    pub scan_workgroup_count: u32,
    pub scan_groups_workgroup_count: u32,
}

impl ExtractComponent for Grass {
    type QueryData = (&'static Grass, &'static GrassGpuInfo, Entity);
    type QueryFilter = ();
    type Out = (Grass, GrassGpuInfo);

    fn extract_component(item: QueryItem<'_, Self::QueryData>) -> Option<Self::Out> {
        Some((item.0.clone(), item.1.clone()))
    }
}

#[derive(Component)]
pub(crate) struct GrassCullChunks(pub HashMap<UVec2, Entity>);
