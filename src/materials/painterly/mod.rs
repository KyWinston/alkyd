use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::materials::painterly::systems::{material_changed, update_material};

use self::resources::PainterlyInspector;

pub mod components;
pub mod painterly;
pub mod resources;
pub mod systems;

pub struct MaterialSwatchPlugin {
    pub debug: bool,
}

impl Plugin for MaterialSwatchPlugin {
    fn build(&self, app: &mut App) {
        if self.debug {
            app.init_resource::<PainterlyInspector>().add_systems(
                Update,
                (update_material, material_changed.after(update_material)),
            );
        }
    }
}
