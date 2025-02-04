use bevy::{pbr::MaterialExtension, prelude::*, render::render_resource::AsBindGroup};
use bevy_easy_compute::prelude::ShaderRef;

use crate::TERRAIN_SHADER_HANDLE;

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct TerrainMaterial {
    #[texture(100)]
    #[sampler(101)]
    pub height_map: Option<Handle<Image>>,
    #[texture(102)]
    #[sampler(103)]
    pub growth_map: Option<Handle<Image>>,
    #[texture(104)]
    #[sampler(105)]
    pub normal_map: Option<Handle<Image>>,
}

impl MaterialExtension for TerrainMaterial {
    fn fragment_shader() -> ShaderRef {
        TERRAIN_SHADER_HANDLE.into()
    }
    fn vertex_shader() -> ShaderRef {
        TERRAIN_SHADER_HANDLE.into()
    }
}
