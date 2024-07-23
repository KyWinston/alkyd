use bevy::{asset::load_internal_asset, prelude::*};
use systems::update_material;

use crate::{Debug, GALAXYFOG_SHADER_HANDLE};

pub mod components;
pub mod galaxy;
pub mod systems;
pub struct GalaxyFogPlugin;

impl Plugin for GalaxyFogPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            GALAXYFOG_SHADER_HANDLE,
            "../../../assets/galaxyfog.wgsl",
            Shader::from_wgsl
        );
        if app.world().resource::<Debug>().0 {
            app.add_systems(Update, update_material);
        }
    }
}
