use alkyd::PIXEL_ART_HANDLE;
use bevy::{asset::load_internal_asset, prelude::*};

pub mod shader;

pub struct PixelArtPlugin;

impl Plugin for PixelArtPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            PIXEL_ART_HANDLE,
            "../../../assets/example_assets/pixel_art.wgsl",
            Shader::from_wgsl
        );
    }
}
