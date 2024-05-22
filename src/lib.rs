use std::path::Path;

use bevy::{
    asset::{embedded_asset, io::AssetSourceId, AssetPath},
    prelude::*,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use compute::plugin::{AppComputePlugin, AppComputeWorkerPlugin};

use materials::painterly::resources::{MaterialsInspector, VoronoiImage};
use utilities::{UtilitiesPlugin, VoronoiWorker};

use crate::{
    materials::painterly::{painterly::PainterlyMaterial, MaterialSwatchPlugin},
    utilities::systems::LoadNoise,
};

pub struct AlkydPlugin {
    pub debug: bool,
}

#[derive(Resource)]
pub struct Debug(pub bool);

pub mod compute;
pub mod materials;
pub mod utilities;

impl Plugin for AlkydPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MaterialSwatchPlugin { debug: self.debug },
            MaterialPlugin::<PainterlyMaterial>::default(),
            UtilitiesPlugin,
            AppComputePlugin,
            EmbeddedAssetPlugin::default(),
            AppComputeWorkerPlugin::<VoronoiWorker>::default(),
        ));
        embedded_asset!(app, "../assets", "../assets/noise.wgsl");
        embedded_asset!(app, "../assets", "../assets/painterly_material.wgsl");

        app.add_event::<LoadNoise>()
            .insert_resource::<VoronoiImage>(VoronoiImage(None))
            .insert_resource::<Debug>(Debug(self.debug))
            .add_systems(Startup, setup);
        if self.debug {
            app.add_plugins(ResourceInspectorPlugin::<MaterialsInspector>::default());
            app.init_resource::<MaterialsInspector>()
                .register_type::<MaterialsInspector>();
        }
    }
}

fn setup() {
    let crate_name = "alkyd";

    let path = Path::new(crate_name).join("noise.wgsl");
    let source = AssetSourceId::from("embedded");
    let asset_path = AssetPath::from_path(&path).with_source(source);

    assert_eq!(asset_path, "embedded://alkyd/noise.wgsl".into());
}
