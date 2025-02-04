use alkyd::{terrain::node::TerrainMaterial, AlkydPlugin};

use bevy::{
    color::palettes::css::GRAY,
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    image::{ImageAddressMode, ImageSamplerDescriptor},
    pbr::ExtendedMaterial,
    prelude::*, scene::SceneInstanceReady,
};

use bevy_third_person_camera::ThirdPersonCameraPlugin;
use iyes_perf_ui::PerfUiPlugin;
use systems::{create_terrain, init_scene};

pub mod systems;
fn main() {
    App::new()
        .insert_resource(ClearColor(GRAY.into()))
        .add_event::<SceneInstanceReady>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin {
                default_sampler: ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    address_mode_w: ImageAddressMode::Repeat,
                    ..Default::default()
                },
            }),
            AlkydPlugin,
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, TerrainMaterial>>::default(),
            FrameTimeDiagnosticsPlugin,
            PerfUiPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            ThirdPersonCameraPlugin,
        ))
        .add_systems(Startup, (init_scene, create_terrain.after(init_scene)))
        .run();
}
