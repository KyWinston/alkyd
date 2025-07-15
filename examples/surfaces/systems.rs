use alkyd::components::Showcase;
use bevy::{
    color::palettes::css::RED,
    core_pipeline::prepass::{DepthPrepass, MotionVectorPrepass, NormalPrepass},
    prelude::*,
    render::camera::CameraProjection,
};
use bevy_third_person_camera::{ThirdPersonCamera, ThirdPersonCameraTarget, Zoom};

use crate::pixel_art::shader::PixelArtMaterial;

pub fn rotate_mesh(mut mesh_q: Query<&mut Transform, With<Showcase>>, time: Res<Time>) {
    if let Ok(mut mesh) = mesh_q.single_mut() {
        mesh.rotate_y(2.0 * time.delta_secs());
    }
}

pub fn init_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        DirectionalLight {
            illuminance: 25000.0,
            ..default()
        },
        Transform::from_xyz(-4.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn_batch([
        (PointLight::default(), Transform::from_xyz(1.0, 3.0, -2.0)),
        (PointLight::default(), Transform::from_xyz(-4.0, 0.5, -2.0)),
    ]);
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            near: 0.5,
            far: 10.0,
            ..default()
        }),
        ThirdPersonCamera {
            zoom: Zoom::new(5.0, 40.0),
            ..default()
        },
        Msaa::Off,
        DepthPrepass,
        NormalPrepass,
        MotionVectorPrepass,
        Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // commands.spawn((
    //     Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(20.0)))),
    //     MeshMaterial3d(materials.add(StandardMaterial::default())),
    //     Transform::from_xyz(0.0, -5.0, -15.0).looking_at(Vec3::new(0.0, 5.0, 15.0), Vec3::Y),
    // ));
}

pub fn create_cube(
    mut commands: Commands,
    mut materials: ResMut<Assets<PixelArtMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material_1 = materials.add(PixelArtMaterial {
        diffuse_color: RED.into(),
        mettalic: 0.0,
        specular: 0.0,
        ..default()
    });

    let mesh = meshes.add(Capsule3d::new(2.0, 4.0));
    commands.spawn((
        Mesh3d(mesh),
        Showcase,
        MeshMaterial3d(material_1),
        ThirdPersonCameraTarget,
        Transform::default().with_rotation(Quat::from_axis_angle(Vec3::Z, 25.0_f32.to_radians())),
    ));
}
