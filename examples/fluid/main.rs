use alkyd::{
    fluid::components::{FluidVolume, VolumeDebug, VolumeFilling},
    AlkydPlugin,
};

use bevy::{
    color::palettes::css::{BLUE, GRAY},
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    image::{ImageAddressMode, ImageSamplerDescriptor},
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
        texture::GpuImage,
    },
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
            MaterialPlugin::<FluidMaterial>::default(),
            PerfUiPlugin,
        ))
        .add_systems(Startup, init_scene)
        .run();
}

pub fn init_scene(mut commands: Commands, mut mesh: ResMut<Assets<Mesh>>) {
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
        Mesh3d(mesh.add(Cuboid::from_length(15.0))),
        FluidVolume::new(Vec3::splat(15.0)),
        VolumeDebug(Timer::from_seconds(5.0, TimerMode::Repeating)),
        InheritedVisibility::VISIBLE,
        VolumeFilling,
        ThirdPersonCameraTarget,
        Transform::from_xyz(0.0, 8.5, 0.0),
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
#[uniform(0, FluidUniform)]
pub struct FluidMaterial {
    pub diffuse_color: Color,
}

impl Default for FluidMaterial {
    fn default() -> Self {
        Self {
            diffuse_color: Color::Srgba(BLUE),
        }
    }
}

#[derive(Clone, ShaderType)]
pub struct FluidUniform {
    pub diffuse_color: Vec4,
}

impl Material for FluidMaterial {
    fn fragment_shader() -> ShaderRef {
        "example_assets/water.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

impl AsBindGroupShaderType<FluidUniform> for FluidMaterial {
    fn as_bind_group_shader_type(&self, _: &RenderAssets<GpuImage>) -> FluidUniform {
        FluidUniform {
            diffuse_color: self.diffuse_color.to_linear().to_vec4(),
        }
    }
}
