use bevy::{
    app::{App, Plugin, Update},
    asset::embedded_asset,
};

use self::resources::{material_changed, PainterlyInspector};

pub mod painterly;
pub mod resources;

pub struct MaterialSwatchPlugin {
    pub debug: bool,
}

impl Plugin for MaterialSwatchPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "src","painterly_material.wgsl");
        if self.debug {
            app.init_resource::<PainterlyInspector>()
                .add_systems(Update, material_changed);
        }
    }
}

