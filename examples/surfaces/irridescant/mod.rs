use alkyd::IRRIDESCANT_SHADER_HANDLE;
use bevy::{asset::load_internal_asset, prelude::*};


pub mod shader;

pub struct IrridescantMaterialPlugin;

impl Plugin for IrridescantMaterialPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            IRRIDESCANT_SHADER_HANDLE,
            "../../../assets/example_assets/irridescant.wgsl",
            Shader::from_wgsl
        );
    }
}
