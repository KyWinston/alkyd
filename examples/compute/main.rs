#[cfg(feature = "compute")]
use alkyd::{workers::resources::Canvas, AlkydPlugin};

use bevy::{
    color::palettes::css::GRAY,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    log::{Level, LogPlugin},
    prelude::*,
    render::texture::{ImageAddressMode, ImageSamplerDescriptor},
};
#[cfg(feature = "compute")]
use systems::{create_cube, init_scene, rotate_mesh};

pub mod systems;

fn main() {
    App::new()
        .insert_resource(ClearColor(GRAY.into()))
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::ERROR,
                    filter: "wgpu=error".to_string(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (1920_f32, 1080_f32).into(),
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
            #[cfg(feature = "compute")]
            AlkydPlugin { debug: false },
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
        ));
    #[cfg(feature = "compute")]
    app.add_systems(Startup, init_scene);
    #[cfg(feature = "compute")]
    app.add_systems(Update, rotate_mesh.after(create_cube))
        .run();
}
