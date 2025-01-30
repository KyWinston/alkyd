use alkyd::AlkydPlugin;

use bevy::{
    color::palettes::css::GRAY, diagnostic::FrameTimeDiagnosticsPlugin, image::{ImageAddressMode, ImageSamplerDescriptor}, pbr::ExtendedMaterial, prelude::*
};

use irridescant::{shader::IrridescantMaterial, IrridescantMaterialPlugin};
use systems::{create_cube, init_scene, rotate_mesh};

pub mod systems;
pub mod irridescant;
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
            IrridescantMaterialPlugin,
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, IrridescantMaterial>>::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, (init_scene, create_cube))
        .add_systems(Update, rotate_mesh.after(create_cube))
        .run();
}
