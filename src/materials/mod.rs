use bevy::app::{App, Plugin, Update};

use self::resources::{material_changed, PainterlyInspector};

pub mod painterly;
pub mod resources;

pub struct MaterialSwatchPlugin;

impl Plugin for MaterialSwatchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, material_changed)
        .init_resource::<PainterlyInspector>();
    }
}
