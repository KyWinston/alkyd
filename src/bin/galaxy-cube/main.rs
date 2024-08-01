use alkyd::{
    components::Showcase, materials::galaxyfog::galaxy::{GalaxyFogMaterial, NoiseProperties}, workers::resources::NoiseImage, AlkydPlugin
};

use bevy::{
    color::palettes::css::PURPLE,
    core_pipeline::prepass::{DepthPrepass, NormalPrepass},
    prelude::*,
    render::texture::{ImageAddressMode, ImageSamplerDescriptor},
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
        .add_systems(Startup, (init_camera.before(init_scene), init_scene))
        .add_systems(
            Update,
            (
                rotate_mesh,
                create_cube.run_if(resource_added::<NoiseImage>),
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
    commands.spawn((Camera3dBundle::default(), DepthPrepass, NormalPrepass));
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
    mut materials: ResMut<Assets<GalaxyFogMaterial>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material = materials.add(GalaxyFogMaterial {
        diffuse_color: Color::srgb_from_array(PURPLE.to_f32_array_no_alpha()),
        radius: 2.0,
        center: Vec3::ZERO,
        steps: 50,
        props: NoiseProperties {
            octaves: 1,
            lacunarity: 2.0,
            frequency: 1.0,
            gain: 0.5,
            amplitude: 1.0,
            ..default()
        },
        ..default()
    });

    let mesh = meshes.add(Cuboid::from_size(Vec3::splat(6.0)));
    commands.spawn((MaterialMeshBundle {
        mesh,
        material,
        ..default()
    },));
}
