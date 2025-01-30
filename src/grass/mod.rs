use bevy::{asset::load_internal_asset, prelude::*, render::render_resource::Shader};

use crate::GRASS_SHADER_HANDLE;

pub mod components;
pub mod pipeline;

pub struct GrassPlugin;

impl Plugin for GrassPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            GRASS_SHADER_HANDLE,
            "../../assets/shaders/grass.wgsl",
            Shader::from_wgsl
        );
    }
}
