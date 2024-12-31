#[cfg(feature = "compute")]
use alkyd::{
    components::Showcase,
    materials::galaxyfog::galaxy::{GalaxyFogMaterial, NoiseProperties},
    workers::resources::NoiseImage,
    AlkydPlugin,
};
use alkyd::{
    components::Showcase,
    materials::galaxyfog::galaxy::{GalaxyFogMaterial, NoiseProperties},
    AlkydPlugin,
};

use bevy::{
    color::palettes::css::PURPLE,
    core_pipeline::prepass::{DepthPrepass, NormalPrepass},
    image::{ImageAddressMode, ImageSamplerDescriptor},
    math::VectorSpace,
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        address_mode_w: ImageAddressMode::Repeat,
                        ..Default::default()
                    },
                })
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..default()
                }),
            AlkydPlugin { debug: true },
        ))
        .add_systems(
            Startup,
            (init_camera.before(init_scene), init_scene, create_cube),
        )
        .add_systems(Update, rotate_mesh)
        .run();
}
#[allow(dead_code)]
fn rotate_mesh(mut mesh_q: Query<&mut Transform, With<Showcase>>, time: Res<Time>) {
    if let Ok(mut mesh) = mesh_q.get_single_mut() {
        mesh.rotate_y(1.0 * time.delta_secs());
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Transform::from_translation(Vec3::new(4.0, 2.0, 5.0)).looking_at(Vec3::ZERO, Vec3::Y),
        Camera3d::default(),
        DepthPrepass,
        NormalPrepass,
    ));
}

fn init_scene(mut commands: Commands) {
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(-4.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    [
        Transform::from_xyz(1.0, 3.0, -2.0),
        Transform::from_xyz(-4.0, 0.5, -2.0),
    ]
    .map(|transform| {
        commands.spawn((PointLight::default(), transform));
    });
}

pub fn create_cube(
    mut materials: ResMut<Assets<GalaxyFogMaterial>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material = materials.add(GalaxyFogMaterial {
        diffuse_color: Color::srgb_from_array(PURPLE.to_f32_array_no_alpha()),
        radius: 1.0,
        center: Vec3::ZERO,
        steps: 25,
        props: NoiseProperties {
            octaves: 2,
            lacunarity: 1.5,
            frequency: 1.0,
            gain: 0.2,
            amplitude: 1.0,
        },
        ..default()
    });
    let mesh = meshes.add(Cuboid::from_size(Vec3::splat(6.0)));
    commands.spawn((Mesh3d(mesh), MeshMaterial3d(material), Showcase));
}
