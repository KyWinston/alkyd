use alkyd::AlkydPlugin;

use bevy::{
    color::palettes::css::GRAY,
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    image::{ImageAddressMode, ImageSamplerDescriptor},
    log::LogPlugin,
    prelude::*,
};

use bevy_third_person_camera::ThirdPersonCameraPlugin;
use iyes_perf_ui::PerfUiPlugin;
use systems::{create_plane, init_scene};

use crate::systems::GrassMaterial;

pub mod systems;
fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(GRAY.into()))
        .add_plugins((
            DefaultPlugins,
            AlkydPlugin,
            MaterialPlugin::<GrassMaterial>::default(),
            PerfUiPlugin,
            FrameTimeDiagnosticsPlugin::default(),
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            ThirdPersonCameraPlugin,
        ))
        .add_systems(Startup, (init_scene, create_plane).chain())
        .run();
}
