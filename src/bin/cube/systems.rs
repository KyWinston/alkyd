use alkyd::{
    materials::painterly::painterly::PainterlyMaterial, workers::{resources::ShaderHandles, systems::make_and_load_shaders2}, Showcase,
};
use bevy::{
    color::palettes::css::BLUE,
    core_pipeline::prepass::{DepthPrepass, NormalPrepass},
    prelude::*,
};

pub fn rotate_mesh(mut mesh_q: Query<&mut Transform, With<Showcase>>, time: Res<Time>) {
    if let Ok(mut mesh) = mesh_q.get_single_mut() {
        mesh.rotate_y(1.0 * time.delta_seconds());
    }
}

pub fn init_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let all_shader_handles: ShaderHandles = make_and_load_shaders2(&asset_server);

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
    mut materials: ResMut<Assets<PainterlyMaterial>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let material = materials.add(PainterlyMaterial {
        diffuse_color: Color::srgb_from_array(BLUE.to_f32_array_no_alpha()),
        brush_handle: Some(asset_server.load("brush_grunge.png")),
        brush_handle_normal: Some(asset_server.load("brush_grunge_normal.png")),
        ..default()
    });

    let mesh = meshes.add(Cuboid::from_size(Vec3::splat(4.0)));
    commands.spawn((
        MaterialMeshBundle {
            mesh,
            material,
            ..default()
        },
        Showcase,
    ));
}
