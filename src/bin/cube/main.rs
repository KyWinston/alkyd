use alkyd::{
    materials::painterly::resources::NoiseImages,
    workers::{systems::setup, DISPLAY_FACTOR, SIZE},
    AlkydPlugin,
};

use bevy::{
    prelude::*,
    render::texture::{ImageAddressMode, ImageSamplerDescriptor},
    utils::hashbrown::HashMap,
};
use systems::{create_cube, init_scene, rotate_mesh};

pub mod systems;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (
                            (SIZE.0 * DISPLAY_FACTOR) as f32,
                            (SIZE.1 * DISPLAY_FACTOR) as f32,
                        )
                            .into(),
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
        ))
        .insert_resource::<NoiseImages>(NoiseImages(HashMap::new()))
        .add_systems(Startup, init_scene.before(setup))
        .add_systems(Update, rotate_mesh.after(create_cube))
        .run();
}
