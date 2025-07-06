use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::{ecs::component::Component, render::storage::ShaderStorageBuffer};
use bevy_app_compute::prelude::{ShaderRef, ShaderType};

use crate::FOLIAGE_GEN_HANDLE;

#[derive(Component, Reflect, Clone, ShaderType, Debug, Default)]
#[reflect(Component)]
pub struct FoliageProperties {
    pub density: f32,
    pub count: u32,
    pub variance: f32,
    pub color: Vec4,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FoliageMaterial {
    #[uniform(0)]
    pub properties: FoliageProperties,
    #[storage(1)]
    pub points: Handle<ShaderStorageBuffer>,
}

impl Material for FoliageMaterial {
    fn vertex_shader() -> ShaderRef {
        FOLIAGE_GEN_HANDLE.into()
    }
    fn specialize(
            pipeline: &bevy::pbr::MaterialPipeline<Self>,
            descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
            layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
            key: bevy::pbr::MaterialPipelineKey<Self>,
        ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}
