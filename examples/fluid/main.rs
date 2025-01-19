use alkyd::{
    fluid::components::{FluidVolume, VolumeDebug, VolumeFilling},
    AlkydPlugin,
};

use bevy::{
    color::palettes::css::GRAY,
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    image::{ImageAddressMode, ImageSamplerDescriptor},
    prelude::*,
};
use bevy_third_person_camera::{
    ThirdPersonCamera, ThirdPersonCameraPlugin, ThirdPersonCameraTarget, Zoom,
};
use iyes_perf_ui::{prelude::PerfUiDefaultEntries, PerfUiPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(GRAY.into()))
        .add_plugins((
            DefaultPlugins.set(ImagePlugin {
                default_sampler: ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    address_mode_w: ImageAddressMode::Repeat,
                    ..Default::default()
                },
            }),
            AlkydPlugin,
            ThirdPersonCameraPlugin,
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            PerfUiPlugin,
        ))
        .add_systems(Startup, init_scene)
        .run();
}

pub fn init_scene(mut commands: Commands) {
    commands.spawn(PerfUiDefaultEntries::default());
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(-4.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ThirdPersonCamera {
            zoom: Zoom::new(20.0, 45.0),
            ..default()
        },
    ));

    commands.spawn((
        FluidVolume::new(500, Vec3::splat(15.0)),
        VolumeDebug,
        InheritedVisibility::VISIBLE,
        VolumeFilling,
        ThirdPersonCameraTarget,
        Transform::from_xyz(0.0, 8.5, 0.0),
    ));
}
