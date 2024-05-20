use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use compute::plugin::{AppComputePlugin, AppComputeWorkerPlugin};
use embedded::EmbeddedAssetPlugin;
use materials::{
    painterly::PainterlyMaterial, resources::MaterialsInspector, MaterialSwatchPlugin,
};
use utilities::{UtilitiesPlugin, VoronoiWorker};

use crate::{materials::resources::VoronoiImage, utilities::systems::LoadNoise};

pub struct AlkydPlugin {
    pub debug: bool,
}

#[derive(Resource)]
pub struct Debug(pub bool);

pub mod compute;
pub mod embedded;
pub mod materials;
pub mod utilities;

impl Plugin for AlkydPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MaterialSwatchPlugin { debug: self.debug },
            EmbeddedAssetPlugin,
            MaterialPlugin::<PainterlyMaterial>::default(),
            UtilitiesPlugin,
            AppComputePlugin,
            AppComputeWorkerPlugin::<VoronoiWorker>::default(),
        ));
        app.add_event::<LoadNoise>()
            .insert_resource::<VoronoiImage>(VoronoiImage(None))
            .insert_resource::<Debug>(Debug(self.debug));
        if self.debug {
            app.add_plugins(ResourceInspectorPlugin::<MaterialsInspector>::default());
            app.init_resource::<MaterialsInspector>()
                .register_type::<MaterialsInspector>();
        }
    }
}
