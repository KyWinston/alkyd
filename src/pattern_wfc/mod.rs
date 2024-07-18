use bevy::{asset::load_internal_asset, prelude::*};

use crate::{PATTERN_FUNC_HANDLE, PATTERN_WFC_HANDLE};

// pub mod components;
pub mod resources;
pub mod shader;
pub mod systems;

pub struct PatternWfcPlugin;

impl Plugin for PatternWfcPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            PATTERN_FUNC_HANDLE,
            "../../assets/shader_utils/tex_gen.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            PATTERN_WFC_HANDLE,
            "../../assets/patterns/pattern_wfc.wgsl",
            Shader::from_wgsl
        );
    }
}
