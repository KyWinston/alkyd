#[cfg(feature = "compute")]
use alkyd::{components::Showcase, workers::systems::make_and_load_shaders};
use bevy::{
    color::palettes::css::WHITE,
    core_pipeline::prepass::{DepthPrepass, NormalPrepass},
    prelude::*,
};
#[cfg(feature = "compute")]
pub fn rotate_mesh(mut mesh_q: Query<&mut Transform, With<Showcase>>, time: Res<Time>) {
    if let Ok(mut mesh) = mesh_q.get_single_mut() {
        mesh.rotate_y(1.0 * time.delta_seconds());
    }
}
#[cfg(feature = "compute")]
pub fn init_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(make_and_load_shaders(&asset_server));
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight::default(),
        transform: Transform::from_xyz(-4.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(1.0, 3.0, -2.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-4.0, 0.5, -2.0),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        DepthPrepass,
        NormalPrepass,
    ));
}

pub fn create_cube(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // asset_server: Res<AssetServer>,
) {
    let material = materials.add(StandardMaterial {
        base_color: WHITE.into(),
        ..default()
    });

    let mesh = meshes.add(Cuboid::from_size(Vec3::splat(4.0)));
    #[cfg(feature = "compute")]
    commands.spawn((
        MaterialMeshBundle {
            mesh,
            material,
            ..default()
        },
        Showcase,
    ));
}
