use bevy::{asset::load_internal_asset, prelude::*, render::render_resource::Shader};

use crate::TERRAIN_SHADER_HANDLE;

pub mod components;
pub mod node;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            TERRAIN_SHADER_HANDLE,
            "../../assets/shaders/terrain.wgsl",
            Shader::from_wgsl
        );
    }
}
