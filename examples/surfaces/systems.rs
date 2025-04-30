use alkyd::components::Showcase;
use bevy::{color::palettes::css::WHITE, pbr::ExtendedMaterial, prelude::*};
use bevy_third_person_camera::{ThirdPersonCamera, ThirdPersonCameraTarget, Zoom};

use crate::irridescant::shader::IrridescantMaterial;

pub fn rotate_mesh(mut mesh_q: Query<&mut Transform, With<Showcase>>, time: Res<Time>) {
    if let Ok(mut mesh) = mesh_q.get_single_mut() {
        mesh.rotate_y(2.0 * time.delta_secs());
    }
}

pub fn init_scene(mut commands: Commands) {
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
}

pub fn create_cube(
    mut commands: Commands,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, IrridescantMaterial>>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material = materials.add(ExtendedMaterial {
        base: StandardMaterial {
            base_color: WHITE.into(),
            ..default()
        },
        extension: IrridescantMaterial { ior: 2.0 },
    });

    let mesh = meshes.add(Capsule3d::new(2.0, 4.0));
    commands.spawn((
        Mesh3d(mesh),
        Showcase,
        MeshMaterial3d(material),
        ThirdPersonCameraTarget,
        Transform::default().with_rotation(Quat::from_axis_angle(Vec3::Z, 25.0_f32.to_radians())),
    ));
}
