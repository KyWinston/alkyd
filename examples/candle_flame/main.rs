use alkyd::{compute::resources::TexGenImage, AlkydPlugin};

use bevy::{
    color::palettes::css::ORANGE,
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    image::{ImageAddressMode, ImageSamplerDescriptor},
    prelude::*,
    window::WindowResolution,
};
use bevy_third_person_camera::{
    ThirdPersonCamera, ThirdPersonCameraPlugin, ThirdPersonCameraTarget, Zoom,
};
use candle_flame::{candle_flame::CandleFlameMaterial, CandleFlamePlugin};
use iyes_perf_ui::{prelude::PerfUiDefaultEntries, PerfUiPlugin};

pub mod candle_flame;

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
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1920., 1080.)
                            .with_scale_factor_override(1.0),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..default()
                }),
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            CandleFlamePlugin,
            PerfUiPlugin,
            ThirdPersonCameraPlugin,
            MaterialPlugin::<CandleFlameMaterial>::default(),
            AlkydPlugin,
        ))
        .add_systems(Startup, (init_camera.before(init_scene), init_scene))
        .add_systems(Update, create_cube.run_if(resource_added::<TexGenImage>))
        .run();
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Transform::from_translation(Vec3::new(5.0, 3.5, 6.0)).looking_at(Vec3::ZERO, Vec3::Y),
        Camera3d::default(),
        ThirdPersonCamera {
            zoom: Zoom::new(5.0, 30.0),
            ..default()
        },
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
    mut materials: ResMut<Assets<CandleFlameMaterial>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    tex_gen: Res<TexGenImage>,
) {
    let material = materials.add(CandleFlameMaterial {
        diffuse_color: Color::srgb_from_array(ORANGE.to_f32_array_no_alpha()),
        radius: 0.9,
        center: Vec3::ZERO,
        steps: 50,
        precision: 50.0,
        fbm: Some(tex_gen.texture_0.clone_weak()),
        fbm_2: Some(tex_gen.texture_1.clone_weak()),
        ..default()
    });
    let mesh = meshes.add(Cuboid::from_size(Vec3::splat(5.0)));
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        ThirdPersonCameraTarget,
    ));
}
