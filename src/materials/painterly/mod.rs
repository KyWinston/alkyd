use bevy::{asset::load_internal_asset, prelude::*};

use crate::PAINTERLY_SHADER_HANDLE;

pub mod painterly;

pub struct MaterialSwatchPlugin;

impl Plugin for MaterialSwatchPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            PAINTERLY_SHADER_HANDLE,
            "../../../assets/painterly_material.wgsl",
            Shader::from_wgsl
        );
    }
}
