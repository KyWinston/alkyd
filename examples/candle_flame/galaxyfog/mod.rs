use bevy::{asset::load_internal_asset, prelude::*};

pub const GALAXYFOG_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1508032910437029714);

pub mod galaxy;
pub mod systems;
pub struct GalaxyFogPlugin;

impl Plugin for GalaxyFogPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            GALAXYFOG_SHADER_HANDLE,
            "../../../assets/example_assets/galaxyfog.wgsl",
            Shader::from_wgsl
        );
    }
}
