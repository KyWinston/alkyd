use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::{ecs::component::Component, render::storage::ShaderStorageBuffer};
use bevy_app_compute::prelude::{AppComputeWorker, ShaderRef, ShaderType};

use crate::SHELL_GEN_HANDLE;
use crate::workers::distribute_points::DistrbutePointsComputeWorker;

#[derive(Component, Reflect, Clone, ShaderType, Debug, Default)]
#[reflect(Component)]
pub struct ShellProperties {
    pub length: f32,
    pub density: f32,
    pub thickness: f32,
    pub count: u32,
    pub attenuation: f32,
    pub dist_attenuation: f32,
    pub occ_bias: f32,
    pub variance: Vec2,
    pub color: Vec4,
}

#[derive(Resource)]
pub struct ShellHandle(pub Handle<ShellMaterial>);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[bind_group_data(ShellMaterialKey)]
pub struct ShellMaterial {
    #[uniform(0)]
    pub properties: ShellProperties,
    pub is_point_cloud: bool,
    #[storage(1)]
    pub point_cloud: Handle<ShaderStorageBuffer>,
}

impl Material for ShellMaterial {
    fn vertex_shader() -> ShaderRef {
        SHELL_GEN_HANDLE.into()
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct ShellMaterialKey {
    pub is_point_cloud: bool,
}

impl From<&ShellMaterial> for ShellMaterialKey {
    fn from(material: &ShellMaterial) -> Self {
        Self {
            is_point_cloud: material.is_point_cloud,
        }
    }
}

pub fn update_points(
    mut worker: ResMut<AppComputeWorker<DistrbutePointsComputeWorker>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    if !worker.ready() {
        return;
    }
}
