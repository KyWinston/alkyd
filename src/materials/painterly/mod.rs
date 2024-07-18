use bevy::{asset::load_internal_asset, prelude::*};
use resources::MaterialsInspector;

use self::resources::PainterlyInspector;

use crate::{
    materials::painterly::systems::{material_changed, update_material},
    PAINTERLY_SHADER_HANDLE,
};

pub mod components;
pub mod painterly;
pub mod resources;
pub mod systems;

pub struct MaterialSwatchPlugin {
    pub debug: bool,
}

impl Plugin for MaterialSwatchPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            PAINTERLY_SHADER_HANDLE,
            "../../../assets/painterly_material.wgsl",
            Shader::from_wgsl
        );
        if self.debug {
            app.init_resource::<PainterlyInspector>()
                .init_resource::<MaterialsInspector>()
                .add_systems(
                    Update,
                    (update_material, material_changed.after(update_material)),
                );
        }
    }
}
