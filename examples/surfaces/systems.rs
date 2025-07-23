use alkyd::{components::Showcase, post_process::pixelate::PixelPostProcessSettings};
use bevy::{
    color::palettes::css::{BLUE, DARK_GREEN},
    core_pipeline::prepass::{DepthPrepass, MotionVectorPrepass, NormalPrepass},
    prelude::*,
};

use crate::pixel_art::shader::PixelArtMaterial;
use bevy_third_person_camera::{ThirdPersonCamera, ThirdPersonCameraTarget, Zoom};

pub fn rotate_mesh(mut mesh_q: Query<&mut Transform, With<Showcase>>, time: Res<Time>) {
    if let Ok(mut mesh) = mesh_q.single_mut() {
        mesh.rotate_y(2.0 * time.delta_secs());
    }
}

pub fn init_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            near: 0.1,
            far: 15.0,
            ..default()
        }),
        PixelPostProcessSettings {
            pixel_size: 4.0,
            ..default()
        },
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
}

pub fn create_cube(
    mut commands: Commands,
    mut materials: ResMut<Assets<PixelArtMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    let material_1 = materials.add(PixelArtMaterial {
        diffuse_color: BLUE.into(),
        outline_color: DARK_GREEN.into(),
        quantize_steps: 5,
        bayer_count: 15,
        bayer_scale: 7.0,
        invert_color: false,
        ..default()
    });

    let mesh = meshes.add(Cuboid::new(5.0, 5.0, 5.0));
    commands.spawn((
        Mesh3d(mesh),
        Showcase,
        MeshMaterial3d(material_1),
        ThirdPersonCameraTarget,
        Transform::default().with_rotation(Quat::from_axis_angle(Vec3::Z, 25.0_f32.to_radians())),
    ));
}
