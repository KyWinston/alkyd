use alkyd::{
    materials::{
        painterly::Painterly,
        resources::{MaterialsInspector, PainterlyInspector},
    },
    AlkydPlugin,
};

use bevy::{
    core_pipeline::prepass::NormalPrepass,
    prelude::*,
    render::texture::{ImageAddressMode, ImageSamplerDescriptor},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin {
                default_sampler: ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    address_mode_w: ImageAddressMode::Repeat,
                    ..Default::default()
                },
            }),
            AlkydPlugin { debug: true },
        ))
        .add_systems(Startup, (init_camera.before(init_scene), init_scene))
        .add_systems(
            Update,
            (rotate_mesh.run_if(resource_exists::<PainterlyInspector>),),
        )
        .run();
}

#[derive(Component)]
struct Showcase;

fn rotate_mesh(
    mut mesh_q: Query<&mut Transform, With<Showcase>>,
    inspector: Res<MaterialsInspector>,
    time: Res<Time>,
) {
    if let Ok(mut mesh) = mesh_q.get_single_mut() {
        if inspector.turn_table {
            mesh.rotate_y(1.0 * time.delta_seconds());
        }
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        NormalPrepass,
    ));
}

fn init_scene(
    mut commands: Commands,
    mut materials: ResMut<Assets<Painterly>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let material = materials.add(Painterly {
        diffuse_color: Color::BLUE,
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
}
