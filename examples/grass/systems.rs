use alkyd::{
    components::Showcase,
    shells::node::{ShellMaterial, ShellProperties},
};
use bevy::{
    color::palettes::css::GREEN, math::VectorSpace, pbr::{ExtendedMaterial, MaterialExtension}, prelude::*, render::{mesh::MeshTag, render_resource::AsBindGroup, storage::ShaderStorageBuffer}
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
    mut materials: ResMut<Assets<ExtendedMaterial<ShellMaterial, GrassMaterial>>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    _asset_server: Res<AssetServer>,
) {
    let mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(5.0)));
    let point_data: [Vec3; 100] = [Vec3::ZERO;100];
    let material = ExtendedMaterial {
        base: ShellMaterial {
            properties: ShellProperties {
                length: 0.15,
                density: 300.0, 
                thickness: 1.0,
                attenuation: 1.0,
                dist_attenuation: 1.0,
                occ_bias: 0.0,
                count: 20,
                variance: Vec2::new(0.0, 1.0),
                color: GREEN.to_vec4(),
            },
            is_point_cloud: false,
            point_cloud: buffers.add(ShaderStorageBuffer::from(point_data)),
        },
        extension: GrassMaterial { displacement: None },
    };

    for i in 0..material.base.properties.count {
        commands.spawn((
            Mesh3d(mesh.clone()),
            Showcase,
            MeshTag(i),
            MeshMaterial3d(materials.add(material.clone())),
            Transform::default(),
            ThirdPersonCameraTarget,
        ));
    }
}

#[derive(Asset, TypePath, AsBindGroup, Default, Clone)]
pub struct GrassMaterial {
    #[texture(100)]
    #[sampler(101)]
    pub displacement: Option<Handle<Image>>,
}

impl MaterialExtension for GrassMaterial {
    fn fragment_shader() -> ShaderRef {
        "example_assets/grass.wgsl".into()
    }
    fn specialize(
        _pipeline: &bevy::pbr::MaterialExtensionPipeline,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::pbr::MaterialExtensionKey<Self>,
    ) -> std::result::Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}
