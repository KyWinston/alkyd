use std::default;

use alkyd::AlkydPlugin;

use bevy::{
    color::palettes::css::GRAY,
    diagnostic::{EntityCountDiagnosticsPlugin, SystemInformationDiagnosticsPlugin},
    image::{ImageAddressMode, ImageSamplerDescriptor},
    pbr::ExtendedMaterial,
    prelude::*,
};

use bevy_third_person_camera::ThirdPersonCameraPlugin;
use irridescant::{IrridescantMaterialPlugin, shader::IrridescantMaterial};
use iyes_perf_ui::PerfUiPlugin;
use systems::{create_cube, init_scene, rotate_mesh};

use crate::pixel_art::{shader::PixelArtMaterial, PixelArtPlugin};

pub mod irridescant;
pub mod pixel_art;
pub mod systems;
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
            AlkydPlugin,
            PixelArtPlugin,
            MaterialPlugin::<PixelArtMaterial>::default(),
            PerfUiPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            ThirdPersonCameraPlugin,
        ))
        .add_systems(Startup, (init_scene, create_cube))
        .add_systems(Update, rotate_mesh.after(create_cube))
        .run();
}
