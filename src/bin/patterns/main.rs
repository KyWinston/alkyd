use alkyd::{
    materials::painterly::resources::VoronoiImage, patterns::shader::PatternGeneratorWfc,
    AlkydPlugin, Showcase,
};

use bevy::{
    core_pipeline::prepass::{DepthPrepass, NormalPrepass},
    prelude::*,
    render::texture::{ImageAddressMode, ImageSamplerDescriptor},
};
use bevy_third_person_camera::{
    camera::Zoom, ThirdPersonCamera, ThirdPersonCameraPlugin, ThirdPersonCameraTarget,
};

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
            ThirdPersonCameraPlugin,
            AlkydPlugin { debug: false },
        ))
        .add_systems(Startup, (init_camera.before(init_scene), init_scene))
        .add_systems(
            Update,
            (
                rotate_mesh,
                create_cube.run_if(resource_added::<VoronoiImage>),
            ),
        )
        .run();
}

fn rotate_mesh(mut mesh_q: Query<&mut Transform, With<Showcase>>, time: Res<Time>) {
    if let Ok(mut mesh) = mesh_q.get_single_mut() {
        mesh.rotate_y(1.0 * time.delta_seconds());
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        ThirdPersonCamera {
            sensitivity: Vec2::new(10.0, 10.0),
            zoom: Zoom::new(4.0, 20.0),
            ..default()
        },
        DepthPrepass,
        NormalPrepass,
    ));
}

fn init_scene(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight::default(),
        transform: Transform::from_xyz(-4.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    [
        Transform::from_xyz(1.0, 3.0, -2.0),
        Transform::from_xyz(-4.0, 0.5, -2.0),
    ]
    .map(|transform| {
        commands.spawn(PointLightBundle {
            transform,
            ..default()
        });
    });
}

pub fn create_cube(
    mut materials: ResMut<Assets<PatternGeneratorWfc>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material = materials.add(PatternGeneratorWfc::default());

    let mesh = meshes.add(Cuboid::from_size(Vec3::splat(6.0)));
    commands.spawn((
        MaterialMeshBundle {
            mesh,
            material,
            ..default()
        },
        ThirdPersonCameraTarget,
    ));
}
