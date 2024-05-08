use bevy::app::{App, Plugin, Update};

use self::resources::{material_changed, PainterlyInspector};

pub mod painterly;
pub mod resources;

pub struct MaterialSwatchPlugin {
    pub debug: bool,
}

impl Plugin for MaterialSwatchPlugin {
    fn build(&self, app: &mut App) {
        if self.debug {
            app.init_resource::<PainterlyInspector>();
            app.add_systems(Update, material_changed);
        }
    }
}
