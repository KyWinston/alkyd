use bevy::{asset::embedded_asset, prelude::*};
use bevy_app_compute::prelude::{AppComputePlugin, AppComputeWorkerPlugin};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use materials::{
    painterly::PainterlyMaterial, resources::MaterialsInspector, MaterialSwatchPlugin,
};

use crate::{
    materials::resources::VoronoiImage,
    utilities::{
        systems::{read_data, run_worker, LoadNoise},
        VoronoiWorker,
    },
};

pub struct AlkydPlugin {
    pub debug: bool,
}

#[derive(Resource)]
pub struct Debug(pub bool);

pub mod materials;
pub mod utilities;

impl Plugin for AlkydPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "src", "utilities/noise.wgsl");
        app.add_plugins((
            MaterialSwatchPlugin { debug: self.debug },
            MaterialPlugin::<PainterlyMaterial>::default(),
            AppComputePlugin,
            AppComputeWorkerPlugin::<VoronoiWorker>::default(),
        ))
        .add_event::<LoadNoise>()
        .insert_resource::<VoronoiImage>(VoronoiImage(None))
        .insert_resource::<Debug>(Debug(self.debug))
        .add_systems(Update, (read_data, run_worker));
        if self.debug {
            app.add_plugins(ResourceInspectorPlugin::<MaterialsInspector>::default());
            app.init_resource::<MaterialsInspector>()
                .register_type::<MaterialsInspector>();
        }
    }
}
