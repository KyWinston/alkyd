use alkyd::{pattern_wfc::shader::PatternGenFunc, utilities::systems::LoadPattern, AlkydPlugin};
use bevy::{
    prelude::*,
    render::{
        extract_component::ExtractComponentPlugin,
        texture::{ImageAddressMode, ImageSamplerDescriptor},
    },
};

fn main() {
    let mut app = App::new();
    app.add_event::<LoadPattern>().add_plugins((
        DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                address_mode_w: ImageAddressMode::Repeat,
                ..Default::default()
            },
        }),
        ExtractComponentPlugin::<PatternGenFunc>::default(),
        AlkydPlugin { debug: false },
    ));

    app.add_systems(Startup, init_camera).run();
}

fn init_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
