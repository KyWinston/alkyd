use alkyd::AlkydPlugin;

use bevy::{
    color::palettes::css::GRAY,
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    image::{ImageAddressMode, ImageSamplerDescriptor},
    prelude::*,
};

use bevy_third_person_camera::ThirdPersonCameraPlugin;
use iyes_perf_ui::PerfUiPlugin;



fn main() {
    App::new()
        .insert_resource(ClearColor(GRAY.into()))
        .add_plugins((
            DefaultPlugins.set(ImagePlugin {
                default_sampler: ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    address_mode_w: ImageAddressMode::Repeat,
                    ..Default::default()
                },
            }),
            ThirdPersonCameraPlugin,
            AlkydPlugin,
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            PerfUiPlugin,
        ))
        .run();
}
