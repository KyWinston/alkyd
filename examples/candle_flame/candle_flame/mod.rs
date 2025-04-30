use bevy::{
    asset::{load_internal_asset, weak_handle},
    prelude::*,
};

pub const GALAXYFOG_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("10efad1a-2db7-4839-ae90-397c34d28d10");

pub mod material;
pub mod systems;
pub struct CandleFlamePlugin;

impl Plugin for CandleFlamePlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            GALAXYFOG_SHADER_HANDLE,
            "../../../assets/example_assets/candle_flame.wgsl",
            Shader::from_wgsl
        );
    }
}
