use alkyd::{workers::resources::Canvas, AlkydPlugin};

use bevy::{
    color::palettes::css::GRAY,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    log::{Level, LogPlugin},
    prelude::*,
    render::texture::{ImageAddressMode, ImageSamplerDescriptor},
};

use systems::{create_cube, init_scene, rotate_mesh};

pub mod systems;

fn main() {
    App::new()
        .insert_resource(ClearColor(GRAY.into()))
        .insert_resource(Canvas {
            width: 1920.0 as u32,
            height: 1080.0 as u32,
            borders: 0.0,
            position: Vec3::ZERO,
        })
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::ERROR,
                    filter: "wgpu=error".to_string(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (1920 as f32, 1080 as f32).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        address_mode_w: ImageAddressMode::Repeat,
                        ..Default::default()
                    },
                }),
            AlkydPlugin { debug: false },
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, init_scene)
        .add_systems(Update, rotate_mesh.after(create_cube))
        .run();
}
