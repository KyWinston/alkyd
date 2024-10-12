use bevy::{asset::load_internal_asset, prelude::*};

use crate::PATTERN_WFC_HANDLE;

// pub mod components;
pub mod resources;
pub mod shader;
pub mod systems;

pub struct PatternWfcPlugin;

impl Plugin for PatternWfcPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            PATTERN_WFC_HANDLE,
            "../../assets/patterns/pattern_wfc.wgsl",
            Shader::from_wgsl
        );
    }
}
