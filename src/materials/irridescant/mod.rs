use bevy::{asset::load_internal_asset, prelude::*};

use crate::IRRIDESCANT_SHADER_HANDLE;

pub mod shader;

pub struct IrridescantMaterialPlugin;

impl Plugin for IrridescantMaterialPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            IRRIDESCANT_SHADER_HANDLE,
            "../../../assets/irridescant.wgsl",
            Shader::from_wgsl
        );
    }
}
