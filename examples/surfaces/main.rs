use std::default;

use alkyd::{
    AlkydPlugin,
    post_process::pixelate::{PixelPostProcessPlugin, PixelPostProcessSettings},
};

use bevy::{
    color::palettes::css::GRAY,
    core_pipeline::post_process::PostProcessingPlugin,
    diagnostic::{EntityCountDiagnosticsPlugin, SystemInformationDiagnosticsPlugin},
    image::{ImageAddressMode, ImageSamplerDescriptor},
    pbr::ExtendedMaterial,
    prelude::*,
    window::WindowResolution,
};

use bevy_third_person_camera::ThirdPersonCameraPlugin;
use iyes_perf_ui::PerfUiPlugin;
use systems::{create_cube, init_scene, rotate_mesh};

use crate::pixel_art::{PixelArtPlugin, shader::PixelArtMaterial};

pub mod irridescant;
pub mod pixel_art;
pub mod systems;
fn main() {
    App::new()
        .insert_resource(ClearColor(GRAY.into()))
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        address_mode_w: ImageAddressMode::Repeat,
                        ..Default::default()
                    },
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1920. / 2.0, 1080. / 2.0)
                            .with_scale_factor_override(1.0),

                        ..default()
                    }),
                    ..default()
                }),
            AlkydPlugin,
            PixelArtPlugin,
            PixelPostProcessPlugin,
            MaterialPlugin::<PixelArtMaterial>::default(),
            PerfUiPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            ThirdPersonCameraPlugin,
        ))
        .add_systems(Startup, init_scene)
        .add_systems(Update, (create_cube, rotate_mesh.after(create_cube)))
        .run();
}
