use alkyd::terrain::node::TerrainMaterial;
use bevy::{color::palettes::css::GREEN, pbr::ExtendedMaterial, prelude::*};
use bevy_third_person_camera::{ThirdPersonCamera, ThirdPersonCameraTarget, Zoom};

#[derive(Resource)]
pub struct TerrainMaps(Handle<Image>, Handle<Image>);

pub fn init_scene(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((
        DirectionalLight {
            illuminance: 25000.0,
            ..default()
        },
        Transform::from_xyz(-4.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((PointLight::default(), Transform::from_xyz(1.0, 3.0, -2.0)));
    commands.spawn((PointLight::default(), Transform::from_xyz(-4.0, 0.5, -2.0)));
    commands.spawn((
        Camera3d::default(),
        ThirdPersonCamera {
            zoom: Zoom::new(5.0, 40.0),
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.insert_resource(TerrainMaps(
        server.load("example_assets/heightmap.png"),
        server.load("example_assets/terrain_normals.png"),
    ));
}

pub fn create_terrain(
    mut commands: Commands,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, TerrainMaterial>>>,
    mut meshes: ResMut<Assets<Mesh>>,
    maps: Res<TerrainMaps>,
) {
    let material = materials.add(ExtendedMaterial {
        base: StandardMaterial {
            base_color: GREEN.into(),
            ..default()
        },
        extension: TerrainMaterial {
            height_map: Some(maps.0.clone()),
            growth_map: None,
            normal_map: Some(maps.1.clone()),
        },
    });

    let mesh = meshes.add(
        Plane3d::new(Vec3::Y, Vec2::splat(7.5))
            .mesh()
            .subdivisions(8),
    );
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        ThirdPersonCameraTarget,
    ));
}
