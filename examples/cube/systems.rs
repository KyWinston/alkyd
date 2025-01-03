use alkyd::components::Showcase;
use bevy::{color::palettes::css::WHITE, prelude::*};

pub fn rotate_mesh(mut mesh_q: Query<&mut Transform, With<Showcase>>, time: Res<Time>) {
    if let Ok(mut mesh) = mesh_q.get_single_mut() {
        mesh.rotate_y(1.0 * time.delta_secs());
    }
}

pub fn init_scene(mut commands: Commands) {
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(-4.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((PointLight::default(), Transform::from_xyz(1.0, 3.0, -2.0)));
    commands.spawn((PointLight::default(), Transform::from_xyz(-4.0, 0.5, -2.0)));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn create_cube(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    // asset_server: Res<AssetServer>,
) {
    let material = materials.add(StandardMaterial {
        base_color: WHITE.into(),
        ..default()
    });

    let mesh = meshes.add(Cuboid::from_size(Vec3::splat(4.0)));
    commands.spawn((Mesh3d(mesh), MeshMaterial3d(material), Showcase));
}
