use alkyd::{SHELL_GEN_HANDLE, components::Showcase};
use bevy::{
    color::palettes::css::GREEN,
    prelude::*,
    render::{
        mesh::MeshTag,
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderType},
        texture::GpuImage,
        view::NoFrustumCulling,
    },
};
use bevy_app_compute::prelude::ShaderRef;
use bevy_third_person_camera::{ThirdPersonCamera, ThirdPersonCameraTarget, Zoom};
use iyes_perf_ui::prelude::PerfUiDefaultEntries;

pub fn init_scene(mut commands: Commands) {
    commands.spawn(PerfUiDefaultEntries::default());
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
        ThirdPersonCamera {
            zoom: Zoom::new(5.0, 40.0),
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn create_plane(
    mut commands: Commands,
    mut materials: ResMut<Assets<GrassMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(15.0)));

    let material = materials.add(GrassMaterial {
        length: 0.15,
        density: 100.0,
        thickness: 2.0,
        shell_count: 16,
        variance: Vec2::new(0.0, 1.0),
        color: GREEN.into(),
    });

    for i in 0..10 {
        commands.spawn((
            Mesh3d(mesh.clone()),
            Showcase,
            MeshTag(i),
            MeshMaterial3d(material.clone()),
            Transform::default(),
            NoFrustumCulling,
            ThirdPersonCameraTarget,
        ));
    }
}

#[derive(Asset, TypePath, AsBindGroup, Default, Clone)]
#[uniform(0, GrassUniform)]
pub struct GrassMaterial {
    pub length: f32,
    pub density: f32,
    pub thickness: f32,
    pub shell_count: u32,
    pub variance: Vec2,
    pub color: Color,
}

#[derive(ShaderType)]
pub struct GrassUniform {
    length: f32,
    density: f32,
    thickness: f32,
    count: u32,
    variance: Vec2,
    color: Vec4,
}

impl Material for GrassMaterial {
    fn vertex_shader() -> ShaderRef {
        SHELL_GEN_HANDLE.into()
    }
    fn fragment_shader() -> ShaderRef {
        "example_assets/grass.wgsl".into()
    }
    fn specialize(
        pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

impl AsBindGroupShaderType<GrassUniform> for GrassMaterial {
    fn as_bind_group_shader_type(&self, _: &RenderAssets<GpuImage>) -> GrassUniform {
        GrassUniform {
            length: self.length,
            density: self.density,
            thickness: self.thickness,
            count: self.shell_count,
            variance: self.variance,
            color: self.color.to_linear().to_vec4(),
        }
    }
}
