use alkyd::AlkydPlugin;

use bevy::{
    color::palettes::css::GRAY,
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    render::texture::{ImageAddressMode, ImageSamplerDescriptor},
};

use systems::{create_cube, init_scene, rotate_mesh};

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
            AlkydPlugin { debug: true },
            FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, (init_scene, create_cube))
        .add_systems(Update, rotate_mesh.after(create_cube))
        .run();
}
