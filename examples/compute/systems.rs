#[cfg(feature = "compute")]
use alkyd::{
    components::Showcase,
    workers::{resources::ShaderHandles, systems::make_and_load_shaders},
};
use bevy::{
    color::palettes::css::BROWN,
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
#[cfg(feature = "compute")]

pub fn rotate_mesh(mut mesh_q: Query<&mut Transform, With<Showcase>>, time: Res<Time>) {
    if let Ok(mut mesh) = mesh_q.get_single_mut() {
        mesh.rotate_y(1.0 * time.delta_seconds());
    }
}
#[cfg(feature = "compute")]

pub fn init_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let all_shader_handles: ShaderHandles = make_and_load_shaders(&asset_server);
    commands.insert_resource(all_shader_handles);

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight::default(),
        transform: Transform::from_xyz(-4.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(1.0, 3.0, -2.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-4.0, 0.5, -2.0),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        DepthPrepass,
        NormalPrepass,
    ));
}

pub fn create_cube(
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, TextureOutputMaterial>>>,
) {
    let _material = materials.add(ExtendedMaterial {
        base: StandardMaterial::default(),
        extension: TextureOutputMaterial {
            color: BROWN.to_vec4(),
        },
    });
    #[cfg(feature = "compute")]
    let mesh = meshes.add(Cuboid::from_size(Vec3::splat(4.0)));
    #[cfg(feature = "compute")]
    commands.spawn((
        MaterialMeshBundle {
            mesh,
            material,
            ..default()
        },
        Showcase,
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct TextureOutputMaterial {
    #[uniform(100)]
    color: Vec4,
}

impl MaterialExtension for TextureOutputMaterial {
    fn fragment_shader() -> ShaderRef {
        "image.wgsl".into()
    }
}
