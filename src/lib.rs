use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use materials::{painterly::Painterly, resources::MaterialsInspector, MaterialSwatchPlugin};

pub struct AlkydPlugin;

pub mod materials;

impl Plugin for AlkydPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MaterialSwatchPlugin,
            MaterialPlugin::<Painterly>::default(),
            ResourceInspectorPlugin::<MaterialsInspector>::default(),
        ))
        .init_resource::<MaterialsInspector>()
        .register_type::<MaterialsInspector>();
    }
}
