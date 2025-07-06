use alkyd::{AlkydPlugin, foliage::node::FoliageMaterial};

use bevy::{
    color::palettes::css::GRAY,
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    pbr::ExtendedMaterial,
    prelude::*,
    scene::SceneInstanceReady,
};

use bevy_third_person_camera::ThirdPersonCameraPlugin;
use iyes_perf_ui::PerfUiPlugin;
use systems::init_scene;

use crate::systems::GrassMaterial;

pub mod systems;
fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(GRAY.into()))
        .add_plugins((
            DefaultPlugins,
            AlkydPlugin,
            MaterialPlugin::<ExtendedMaterial<FoliageMaterial, GrassMaterial>>::default(),
            PerfUiPlugin,
            FrameTimeDiagnosticsPlugin::default(),
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            ThirdPersonCameraPlugin,
        ))
        .add_event::<SceneInstanceReady>()
        .add_systems(Startup, init_scene)
        .run();
}
