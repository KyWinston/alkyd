use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::embedded_asset,
    ecs::schedule::IntoSystemConfigs,
};

use crate::materials::resources::init_material;

use self::resources::{material_changed, PainterlyInspector};

pub mod components;
pub mod painterly;
pub mod resources;

pub struct MaterialSwatchPlugin {
    pub debug: bool,
}

impl Plugin for MaterialSwatchPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "src", "painterly_material.wgsl");
        if self.debug {
            app.init_resource::<PainterlyInspector>()
                .add_systems(Startup, init_material)
                .add_systems(
                    Update,
                    (init_material, material_changed.after(init_material)),
                );
        }
    }
}
