use alkyd::AlkydPlugin;

use bevy::{
    prelude::*,
    render::texture::{ImageAddressMode, ImageSamplerDescriptor},
};
use systems::{create_cube, rotate_mesh};

pub mod systems;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin {
                default_sampler: ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    address_mode_w: ImageAddressMode::Repeat,
                    ..Default::default()
                },
            }),
            AlkydPlugin { debug: false },
        ))
        // .insert_resource::<NoiseImage>(NoiseImages(HashMap::new()))
        // .add_systems(Startup, init_scene.before(setup))
        .add_systems(Update, rotate_mesh.after(create_cube))
        .run();
}
