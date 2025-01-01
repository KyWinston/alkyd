use alkyd::{components::Showcase, AlkydPlugin};

use bevy::{
    color::palettes::css::ORANGE, core_pipeline::prepass::{DepthPrepass, NormalPrepass}, diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin}, image::{ImageAddressMode, ImageSamplerDescriptor}, prelude::*
};
use galaxyfog::{
    galaxy::{GalaxyFogMaterial, NoiseProperties},
    GalaxyFogPlugin,
};
use iyes_perf_ui::{prelude::PerfUiDefaultEntries, PerfUiPlugin};

pub mod galaxyfog;

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
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            GalaxyFogPlugin,
            PerfUiPlugin,
            MaterialPlugin::<GalaxyFogMaterial>::default(),
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
        Transform::from_translation(Vec3::new(5.0, 3.5, 6.0)).looking_at(Vec3::ZERO, Vec3::Y),
        Camera3d::default(),
        DepthPrepass,
        NormalPrepass,
    ));
}

fn init_scene(mut commands: Commands) {
    commands.spawn(PerfUiDefaultEntries::default());
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
        diffuse_color: Color::srgb_from_array(ORANGE.to_f32_array_no_alpha()),
        radius: 0.7,
        center: Vec3::ZERO,
        steps: 50,
        props: NoiseProperties {
            octaves: 2,
            lacunarity: 2.0,
            frequency: 1.0,
            gain: 0.2,
            amplitude: 1.0,
        },
        ..default()
    });
    let mesh = meshes.add(Cuboid::from_size(Vec3::splat(6.0)));
    commands.spawn((Mesh3d(mesh), MeshMaterial3d(material), Showcase));
}
