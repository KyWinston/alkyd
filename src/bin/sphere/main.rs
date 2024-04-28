use alkyd::{materials::painterly::Painterly, AlkydPlugin};
use bevy::{core_pipeline::prepass::NormalPrepass, math::primitives::Sphere, prelude::*};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AlkydPlugin))
        .add_systems(Startup, (init_camera.before(init_scene), init_scene))
        .run();
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),

            ..default()
        },
        NormalPrepass,
    ));
}

fn init_scene(
    mut commands: Commands,
    mut materials: ResMut<Assets<Painterly>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material = materials.add(Painterly {
        diffuse_color: Color::BLUE,
        ..default()
    });

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Sphere::new(4.0)),
        material,
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight::default(),
        transform: Transform::from_xyz(0.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(1.0, 3.0, -2.0),
        ..default()
    });
}
